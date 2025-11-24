/// 朔望节气拟合参数
/// 包含起始儒略日和周期天数

// 拟合参数结构体
pub(crate) struct FitParameter {
    pub start_julian_day: f64, // 起始儒略日
    pub period_days: f64,      // 周期天数
}

pub const SHUO_FIT_PARAMETERS: &[FitParameter] = &[
    FitParameter {
        start_julian_day: 1457698.231017,
        period_days: 29.53067166,
    }, // -721-12-17
    FitParameter {
        start_julian_day: 1546082.512234,
        period_days: 29.53085106,
    }, // -479-12-11
    FitParameter {
        start_julian_day: 1640640.735300,
        period_days: 29.53060000,
    }, // -221-10-31
    FitParameter {
        start_julian_day: 1642472.151543,
        period_days: 29.53085439,
    }, // -216-11-04
    FitParameter {
        start_julian_day: 1683430.509300,
        period_days: 29.53086148,
    }, // -104-12-25
    FitParameter {
        start_julian_day: 1752148.041079,
        period_days: 29.53085097,
    }, //   85-02-13
    FitParameter {
        start_julian_day: 1807665.420323,
        period_days: 29.53059851,
    }, //  237-02-12
    FitParameter {
        start_julian_day: 1883618.114100,
        period_days: 29.53060000,
    }, //  445-01-24
    FitParameter {
        start_julian_day: 1907360.704700,
        period_days: 29.53060000,
    }, //  510-01-26
    FitParameter {
        start_julian_day: 1936596.224900,
        period_days: 29.53060000,
    }, //  590-02-10
    FitParameter {
        start_julian_day: 1939135.675300,
        period_days: 29.53060000,
    }, //  597-01-24
];

pub const QI_FIT_PARAMETERS: &[FitParameter] = &[
    FitParameter {
        start_julian_day: 1640650.479938,
        period_days: 15.21842500,
    }, // -221-11-09
    FitParameter {
        start_julian_day: 1642476.703182,
        period_days: 15.21874996,
    }, // -216-11-09
    FitParameter {
        start_julian_day: 1683430.515601,
        period_days: 15.218750011,
    }, // -104-12-25
    FitParameter {
        start_julian_day: 1752157.640664,
        period_days: 15.218749978,
    }, //   85-02-23
    FitParameter {
        start_julian_day: 1807675.003759,
        period_days: 15.218620279,
    }, //  237-02-22
    FitParameter {
        start_julian_day: 1883627.765182,
        period_days: 15.218612292,
    }, //  445-02-03
    FitParameter {
        start_julian_day: 1907369.128100,
        period_days: 15.218449176,
    }, //  510-02-03
    FitParameter {
        start_julian_day: 1936603.140413,
        period_days: 15.218425000,
    }, //  590-02-17
    FitParameter {
        start_julian_day: 1939145.524180,
        period_days: 15.218466998,
    }, //  597-02-03
    FitParameter {
        start_julian_day: 1947180.798300,
        period_days: 15.218524844,
    }, //  619-02-03
    FitParameter {
        start_julian_day: 1964362.041824,
        period_days: 15.218533526,
    }, //  666-02-17
    FitParameter {
        start_julian_day: 1987372.340971,
        period_days: 15.218513908,
    }, //  729-02-16
    FitParameter {
        start_julian_day: 1999653.819126,
        period_days: 15.218530782,
    }, //  762-10-03
    FitParameter {
        start_julian_day: 2007445.469786,
        period_days: 15.218535181,
    }, //  784-02-01
    FitParameter {
        start_julian_day: 2021324.917146,
        period_days: 15.218526248,
    }, //  822-02-01
    FitParameter {
        start_julian_day: 2047257.232342,
        period_days: 15.218519654,
    }, //  893-01-31
    FitParameter {
        start_julian_day: 2070282.898213,
        period_days: 15.218425000,
    }, //  956-02-16
    FitParameter {
        start_julian_day: 2073204.872850,
        period_days: 15.218515221,
    }, //  964-02-16
    FitParameter {
        start_julian_day: 2080144.500926,
        period_days: 15.218530782,
    }, //  983-02-16
    FitParameter {
        start_julian_day: 2086703.688963,
        period_days: 15.218523776,
    }, // 1001-01-31
    FitParameter {
        start_julian_day: 2110033.182763,
        period_days: 15.218425000,
    }, // 1064-12-15
    FitParameter {
        start_julian_day: 2111190.300888,
        period_days: 15.218425000,
    }, // 1068-02-15
    FitParameter {
        start_julian_day: 2113731.271005,
        period_days: 15.218515671,
    }, // 1075-01-30
    FitParameter {
        start_julian_day: 2120670.840263,
        period_days: 15.218425000,
    }, // 1094-01-30
    FitParameter {
        start_julian_day: 2123973.309063,
        period_days: 15.218425000,
    }, // 1103-02-14
    FitParameter {
        start_julian_day: 2125068.997336,
        period_days: 15.218477932,
    }, // 1106-02-14
    FitParameter {
        start_julian_day: 2136026.312633,
        period_days: 15.218472436,
    }, // 1136-02-14
    FitParameter {
        start_julian_day: 2156099.495538,
        period_days: 15.218425000,
    }, // 1191-01-29
    FitParameter {
        start_julian_day: 2159021.324663,
        period_days: 15.218425000,
    }, // 1199-01-29
    FitParameter {
        start_julian_day: 2162308.575254,
        period_days: 15.218461742,
    }, // 1208-01-30
    FitParameter {
        start_julian_day: 2178485.706538,
        period_days: 15.218425000,
    }, // 1252-05-15
    FitParameter {
        start_julian_day: 2178759.662849,
        period_days: 15.218445786,
    }, // 1253-02-13
    FitParameter {
        start_julian_day: 2185334.020800,
        period_days: 15.218425000,
    }, // 1271-02-13
    FitParameter {
        start_julian_day: 2187525.481425,
        period_days: 15.218425000,
    }, // 1277-02-12
    FitParameter {
        start_julian_day: 2188621.191481,
        period_days: 15.218437494,
    }, // 1280-02-13
];
