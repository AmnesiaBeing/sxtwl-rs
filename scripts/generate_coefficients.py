def process_precession_table(input_str):
    lines = input_str.strip().split('\n')
    processed_lines = []

    comment_addr = 0
    
    for line in lines:
        # 初始化注释部分
        comment_part = ""
        data_part = line
        
        # 处理行内 /* */ 注释
        if '/*' in line and '*/' in line:
            # 找到注释的开始和结束位置
            comment_start = line.find('/*')
            comment_end = line.find('*/') + 2
            
            # 提取注释内容
            comment_part = line[comment_start:comment_end]

            comment_addr = 1
            
            # 从数据部分移除注释
            data_part = line[:comment_start] + line[comment_end:]
        
        # 处理行末 // 注释
        if '//' in data_part:
            parts = data_part.split('//', 1)
            data_part = parts[0]
            if comment_part:
                comment_part += ' //' + parts[1]
            else:
                comment_part = '//' + parts[1]
        
        # 处理数字部分
        numbers = []
        for num_str in data_part.split(','):
            num_str = num_str.strip()
            if not num_str:
                continue
                
            # 去掉正号
            if num_str.startswith('+'):
                num_str = num_str[1:]
            
            # 整数后面加.0
            if '.' not in num_str and num_str.lstrip('-').isdigit():
                num_str += '.0'
            
            numbers.append(num_str)
        
        # 重新组合行
        processed_line = '    ' + ', '.join(numbers)
        if comment_part and comment_addr == 0:
            processed_line += ', ' + comment_part
        elif comment_part and comment_addr == 1:
            processed_line = comment_part + processed_line + ', '
        processed_lines.append(processed_line)
    
    # 构建最终的Rust代码
    rust_code = '''{}'''.format('\n'.join(processed_lines))
    
    return rust_code

# 测试数据
input_data = '''	//经(角秒),纬(角秒), 距(10-6AU)
	-0.08631, +0.00039, -0.00008,  //水星
	-0.07447, +0.00006, +0.00017,  //金星
	-0.07135, -0.00026, -0.00176,  //火星
	-0.20239, +0.00273, -0.00347,  //木星
	-0.25486, +0.00276, +0.42926,  //土星
	+0.24588, +0.00345, -14.46266, //天王星
	-0.95116, +0.02481, +58.30651  //海王星
'''

# 处理并输出结果
result = process_precession_table(input_data)
print(result)