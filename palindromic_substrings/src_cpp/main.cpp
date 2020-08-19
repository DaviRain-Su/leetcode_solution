//
// Created by 苏胤榕 on 2020/8/19.
//

#include <iostream>
#include <algorithm>
#include <string>

using  namespace  std;
bool is_palindromic(string s);

int countSubstrings(string s) {
    int length  = s.length();
//    cout << "length = " << length << endl;
    int count = 0;
    for (int gap = 1; gap <= length; gap++){
        for (int index = 0; index < length; index++){
            if (length < index + gap)  {
                continue;
            }
            string  inner_str = s.substr(index, gap);
//            cout << "inner_str = " << inner_str << endl;
            if (is_palindromic(inner_str)) {
                count += 1;
            }
        }
    }
    return count;
}

bool is_palindromic(string s) {
    int length = s.length();
    for (int i = 0; i < length/2; i++){
        string lhs = s.substr(i,1);
        string rhs = s.substr(length-i-1,1);
        if (lhs == rhs) {
            continue;
        }else {
            return false;
        }
    }
    return true;
}


int main() {
    string  s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    int ret = countSubstrings(s);
//    bool ret = is_palindromic(s);
    cout << "ret = " << ret << endl;
    return 0;
}