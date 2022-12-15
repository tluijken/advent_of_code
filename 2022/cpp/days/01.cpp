#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <string.h>
#include <vector>
using namespace std;

int main() {
  string s;
  ifstream dataFile("../../rust/input/2022/day1.txt");
  std::vector<int> v;
  int totalGroupVal = 0;
  while (getline(dataFile, s)) {
    if (s.empty() || !s.find_first_not_of("0123456789")) {
      v.push_back(totalGroupVal);
      totalGroupVal = 0;
    } else {
      totalGroupVal += stoi(s);
    }
  }
  sort(v.begin(), v.end(), greater<int>());
  std::cout << "Day 1 part one : " << v[0] << "\n";
  std::cout << "Day 1 part two : " << v[0] + v[1] + v[2] << "\n";

  return 0;
}
