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
  vector<string> lines;

  // read all the lines
  while (getline(dataFile, s)) {
    lines.push_back(s);
  }

  // Day 1
  for (int i = 0; i < lines.size(); i++) {
    string s = lines[i];
    string firstHalf = s.substr(0, s.length() / 2);
    string secondHalf = s.substr(s.length() / 2, s.length());
    for (int i = 0; i < secondHalf.length(); i++) {
      if (firstHalf.find(secondHalf[i]) != std::string::npos) {
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

  // Day 2
  priority = 0;
  for (int i = 0; i < lines.size(); i += 3) {
    for (int j = 0; j < lines[i].length(); j++) {
      if (lines[i + 1].find(lines[i][j]) != std::string::npos &&
          lines[i + 2].find(lines[i][j]) != std::string::npos) {
        int d = (int)lines[i][j];
        if (d <= 90) {
          priority += d - 38;
        } else {
          priority += d - 96;
        }
      }
    }
  }
  cout << "Day 3 part 2 : " << priority << "\n";

  return 0;
}
