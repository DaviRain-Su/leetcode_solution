//
// Created by 苏胤榕 on 2020/8/19.
//
/*


learn string operator


1. 截取子串

       s.substr(pos, n)    截取s中从pos开始（包括0）的n个字符的子串，并返回

       s.substr(pos)        截取s中从从pos开始（包括0）到末尾的所有字符的子串，并返回


2. 替换子串

       s.replace(pos, n, s1)    用s1替换s中从pos开始（包括0）的n个字符的子串


3. 查找子串

       s.find(s1)         查找s中第一次出现s1的位置，并返回（包括0）

       s.rfind(s1)        查找s中最后次出现s1的位置，并返回（包括0）

       s.find_first_of(s1)       查找在s1中任意一个字符在s中第一次出现的位置，并返回（包括0）

       s.find_last_of(s1)       查找在s1中任意一个字符在s中最后一次出现的位置，并返回（包括0）

       s.fin_first_not_of(s1)         查找s中第一个不属于s1中的字符的位置，并返回（包括0）

       s.fin_last_not_of(s1)         查找s中最后一个不属于s1中的字符的位置，并返回（包括0）


 * */



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