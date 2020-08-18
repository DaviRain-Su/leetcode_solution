//
// Created by 苏胤榕 on 2020/8/18.
//
#include <iostream>
#include <algorithm>
#include <vector>
#include <array>

using namespace  std;

struct Sum {
    void operator()(int n) {
        sum += n;
    }

    Sum(): sum(0){}

    int sum;
};

int pivotIndex(vector<int>& nums) {
    Sum s = for_each(nums.begin(), nums.end(), Sum());
    int left_sum = 0;
    for(auto i = 0; i < nums.size(); i++){
        if (left_sum == s.sum - nums[i] - left_sum) {
            return i;
        }
        left_sum += nums[i];
    }
    return -1;
}

int main() {
    int temp[] = {1, 7, 3, 6, 5, 6};
    vector<int> vec;
    for (int i = 0; i < sizeof(temp) / sizeof(int); i++){
        vec.push_back(temp[i]);
    }
    int ret = pivotIndex(vec);
    cout << "ret = " << ret << endl;
    return 0;
}

