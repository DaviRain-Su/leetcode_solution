from typing import List


def pivot_index(nums: List[int]) -> int:
    s = sum(nums)
    left_sum = 0
    for i, x in enumerate(nums):
        if left_sum == (s - left_sum - x):
            return i
        left_sum += x
    return -1


if __name__ == "__main__":
    # temp_list = [1, 7, 3, 6, 5, 6]
    # temp_list = []
    temp_list = [1, 2, 3]
    ret = pivot_index(temp_list)
    print(ret)
