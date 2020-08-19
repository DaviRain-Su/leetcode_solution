# 代码超时
#
#
# python string slice
# 
# 切片操作（slice）可以从一个字符串中获取子字符串（字符串的一部分）。我们使用一对方括号、起始偏移量start、终止偏移量end 以及可选的步长step 来定义一个分片。
#
# 格式： [start:end:step]
#
# • [:] 提取从开头（默认位置0）到结尾（默认位置-1）的整个字符串
# • [start:] 从start 提取到结尾
# • [:end] 从开头提取到end - 1
# • [start:end] 从start 提取到end - 1
# • [start:end:step] 从start 提取到end - 1，每step 个字符提取一个
# • 左侧第一个字符的位置/偏移量为0，右侧最后一个字符的位置/偏移量为-1
#
#
#
# 几个特别的examples 如下：
#
# 提取最后N个字符：
#
# >>> letter = 'abcdefghijklmnopqrstuvwxyz'
# >>> letter[-3:]
# 'xyz'
#
# 从开头到结尾，step为N：
#
# >>> letter[::5]
# 'afkpuz'
#
# 将字符串倒转(reverse)， 通过设置步长为负数：
#
# >>> letter[::-1]
# 'zyxwvutsrqponmlkjihgfedcba'



def is_palindromic(s: str) -> bool:
    length: int = len(s)
    for i in range(length // 2):
        lhs = s[i:i + 1]
        rhs = s[length - i - 1:length - i]
        if lhs == rhs:
            continue
        else:
            return False
    return True


def countSubstrings(s: str) -> int:
    # def is_palindromic(s: str) -> bool:
    #     _length: int = len(s)
    #     for i in range(_length // 2):
    #         lhs = s[i:i + 1]
    #         rhs = s[_length - i - 1:_length - i]
    #         if lhs == rhs:
    #             continue
    #         else:
    #             return False
    #     return True

    length: int = len(s)
    count = 0
    for gap in range(length):
        new_gap = gap + 1
        for index in range(length):
            if length < index + new_gap:
                continue
            inner_str = s[index:index + new_gap]
            if is_palindromic(inner_str):
                count += 1

    return count


if __name__ == "__main__":
    # print("is_palindromic {}".format(is_palindromic("abc")))

    print("ret = {}".format(countSubstrings("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")))
