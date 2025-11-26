#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

// 缓存条目
#[derive(Clone, Copy)]
pub struct CacheEntry<K, V, const DATA_SIZE: usize>
where
    K: Copy + PartialEq,
    V: Copy,
{
    pub key: K,
    pub data: [V; DATA_SIZE],
    pub access_count: usize,
}

// 固定大小LRU缓存
pub struct FixedCache<K, V, const CACHE_SIZE: usize, const DATA_SIZE: usize>
where
    K: Copy + PartialEq,
    V: Copy,
{
    entries: [Option<CacheEntry<K, V, DATA_SIZE>>; CACHE_SIZE],
    lru_counter: AtomicUsize,
}

impl<K, V, const CACHE_SIZE: usize, const DATA_SIZE: usize> FixedCache<K, V, CACHE_SIZE, DATA_SIZE>
where
    K: Copy + PartialEq,
    V: Copy,
{
    pub const fn new() -> Self {
        Self {
            entries: [None; CACHE_SIZE],
            lru_counter: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, key: K) -> Option<[V; DATA_SIZE]> {
        // 使用原子操作确保内存可见性
        core::sync::atomic::fence(Ordering::Acquire);

        for entry in &self.entries {
            if let Some(entry) = entry {
                if entry.key == key {
                    // 更新访问计数（原子操作）
                    self.lru_counter.fetch_add(1, Ordering::Relaxed);
                    return Some(entry.data);
                }
            }
        }
        None
    }

    pub fn insert(&mut self, key: K, data: [V; DATA_SIZE]) {
        // 使用原子操作确保内存可见性
        core::sync::atomic::fence(Ordering::Release);

        let new_entry = CacheEntry {
            key,
            data,
            access_count: self.lru_counter.load(Ordering::Relaxed),
        };

        // 查找空位或最旧的条目
        let mut replace_index = 0;
        let mut oldest_access = usize::MAX;

        for (i, entry) in self.entries.iter().enumerate() {
            match entry {
                None => {
                    // 找到空位
                    self.entries[i] = Some(new_entry);
                    return;
                }
                Some(entry) if entry.access_count < oldest_access => {
                    // 记录最旧的条目
                    oldest_access = entry.access_count;
                    replace_index = i;
                }
                _ => {}
            }
        }

        // 替换最旧的条目
        self.entries[replace_index] = Some(new_entry);
    }

    pub fn clear(&mut self) {
        self.entries = [None; CACHE_SIZE];
        self.lru_counter.store(0, Ordering::Relaxed);
    }

    pub fn len(&self) -> usize {
        self.entries.iter().filter(|e| e.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|e| e.is_none())
    }
}

// 线程安全的缓存包装器
pub struct ThreadSafeCache<K, V, const CACHE_SIZE: usize, const DATA_SIZE: usize>
where
    K: Copy + PartialEq,
    V: Copy,
{
    cache: Mutex<FixedCache<K, V, CACHE_SIZE, DATA_SIZE>>,
}

impl<K, V, const CACHE_SIZE: usize, const DATA_SIZE: usize>
    ThreadSafeCache<K, V, CACHE_SIZE, DATA_SIZE>
where
    K: Copy + PartialEq + core::fmt::Debug,
    V: Copy + core::fmt::Debug,
{
    pub const fn new() -> Self {
        Self {
            cache: Mutex::new(FixedCache::new()),
        }
    }

    pub fn get(&self, key: K) -> Option<[V; DATA_SIZE]> {
        let cache = self.cache.lock();
        cache.get(key)
    }

    pub fn insert(&self, key: K, data: [V; DATA_SIZE]) {
        let mut cache = self.cache.lock();
        cache.insert(key, data);
    }

    pub fn get_or_compute<F>(&self, key: K, compute_fn: F) -> [V; DATA_SIZE]
    where
        F: FnOnce() -> [V; DATA_SIZE],
    {
        // 先尝试从缓存获取
        if let Some(data) = self.get(key) {
            return data;
        }

        // 缓存未命中，进行计算
        let data = compute_fn();

        // 存入缓存
        self.insert(key, data);

        data
    }

    pub fn clear(&self) {
        let mut cache = self.cache.lock();
        cache.clear();
    }

    pub fn len(&self) -> usize {
        let cache = self.cache.lock();
        cache.len()
    }

    pub fn is_empty(&self) -> bool {
        let cache = self.cache.lock();
        cache.is_empty()
    }
}

// 便捷宏，用于快速创建缓存实例
#[macro_export]
macro_rules! create_cache {
    ($name:ident, $key_type:ty, $value_type:ty, $cache_size:expr, $data_size:expr) => {
        static $name: $crate::cache::ThreadSafeCache<
            $key_type,
            $value_type,
            $cache_size,
            $data_size,
        > = $crate::cache::ThreadSafeCache::new();
    };
}
