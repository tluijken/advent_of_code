#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

int main() {
  string s;
  ifstream dataFile("../../rust/input/2022/day1.txt");
  vector<int> v;
  int totalGroupVal = 0;
  while (getline(dataFile, s)) {
    if (s.empty() || !all_of(s.begin(), s.end(), ::isdigit)) {
      v.push_back(totalGroupVal);
      totalGroupVal = 0;
    } else {
      totalGroupVal += stoi(s);
    }
  }
  sort(v.begin(), v.end(), greater<int>());
  cout << "Day 1 part one : " << v[0] << "\n";
  cout << "Day 1 part two : " << v[0] + v[1] + v[2] << "\n";

  return 0;
}
