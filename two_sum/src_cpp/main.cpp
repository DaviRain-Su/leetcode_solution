#include <iostream>
#include <map>
#include <vector>
#include <algorithm>

using namespace std;

vector<int> twoSum(vector<int>& nums, int target) {
    map<int64_t, int64_t> _map;
    vector<int> res;

    for(auto i = 0; i < nums.size(); i++) {
        _map[nums[i]] = i;
    }

    for(auto j = 0; j < nums.size(); j++) {
        int temp = target - nums[j];
        if(_map.find(temp) != _map.end() && _map.at(temp) != j) {
            res.push_back(j);
            res.push_back(_map.at(temp));
            break;
        }
    }
    return res;
}

vector<int> twoSum_v2(vector<int> &nums, int target) {
    
    /*
    vector<int> v;
    for (int i = 0; i < nums.size(); i++){
        for(int j = i + 1; j < nums.size(); j++){
            if(nums[i] + nums[j] == target) {
                v.push_back(i);
                v.push_back(j);

            }
        }
    }
    return v; 
    */
    
    /**
     *
     * unordered_map<int, int> _map;
     * vector<int> v;
     * for(int i = 0; i < nums.size(); i++){
     *  int temp = target - nums[i];
     *  if (_map.find(temp) != _map.end()) {
     *      v.push_back(_map[temp]);
     *      v.push_back(i);
     *      return v;
     *  }
     *  _map[num[i]] = i;
     * }
     *
     * return v;
     *
     * 
     **/
    
    map<int, int> _map;
    for (auto i = 0; i < nums.size(); i++) {
        int temp = target - nums[i];
        if(_map.find(temp) != _map.end()) {
            return {_map[temp], i};
        }
        _map[nums[i]] = i;
    }
    return {};
}
int main()
{
    int array[] = {2, 7, 9, 34,4};
    vector<int> nums;
    for(int i = 0; i < 5; i++){
        nums.push_back(array[i]);
    }
    
    int target = 9;
    auto ret = twoSum_v2(nums, target);
    for ( auto i: ret ){
        cout << i << " ";
    }
    return 0;
}

