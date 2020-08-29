#include <iostream>
#include <vector>
#include <string>
#include <cassert>
#include <algorithm>

using namespace std;

auto numJewelsInStones(string J, string S) -> int {
    int count = 0;
    for_each(S.begin(), S.end(), [&count, J](const char& s) {
         for_each(J.begin(), J.end(), [&count,s](const char& j) {
            if (s == j) { count++; }
        });
    });
    return count;
}

int main() {
    
    //assert(numJewelsInStones(string("aA"), string("aAAAbbbb")) == 3);
    cout << "result = " << numJewelsInStones(string("aA"), string("aAAAbbb")) << endl; 
    return 0;
}
