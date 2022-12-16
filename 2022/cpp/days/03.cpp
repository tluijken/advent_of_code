#include <algorithm>
#include <cctype>
#include <fstream>
#include <functional>
#include <iostream>
#include <string.h>
#include <string>
#include <vector>
using namespace std;

int main() {
  string s;
  ifstream dataFile("../../rust/input/2022/day3.txt");
  int priority = 0;
  while (getline(dataFile, s)) {
    string firstHalf = s.substr(0, s.length() / 2);
    string secondHalf = s.substr(s.length() / 2, s.length());
    for (int i = 0; i < secondHalf.length(); i++) {
      if (firstHalf.find(secondHalf[i]) != std::string::npos) {
        cout << "Found " << secondHalf[i] << " in " << firstHalf << "\n";
        int d = (int)secondHalf[i];
        if (d <= 90) {
          priority += d - 38;
        } else {
          priority += d - 96;
        }
        break;
      }
    }
  }

  cout << "Day 3 part 1 : " << priority << "\n";

  return 0;
}
