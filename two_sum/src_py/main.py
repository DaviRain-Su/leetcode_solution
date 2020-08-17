#!/usr/bin/env python3
# coding=utf-8
from typing import List
def twoSum(nums: List[int], target: int) -> List[int]:
    dict = {};
    for i in range(len(nums)):
        if (target - nums[i]) in dict :
            return [dict[target - nums[i]], i]
        dict[nums[i]] = i


if __name__ == "__main__":

    nums: List[int] = [2, 7, 3, 4, 5];
    target: int = 9;

    ret = twoSum(nums, target);

    print(ret);
