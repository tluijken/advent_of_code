#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>
using namespace std;

bool allUniqueChars(string str) {
  for (int i = 0; i < str.length(); i++) {
    for (char &c : str.substr(i + 1))
      if (str[i] == c) {
        return false;
      }
  }
  return true;
}

int getDistinctSequence(string &input, int windowSize) {
  for (int i = 0; i < input.length(); i++) {
    if (allUniqueChars(input.substr(i, windowSize))) {
      return i + windowSize;
    }
  }
  return 0;
}

int main() {
  string s;
  ifstream dataFile("../../rust/input/2022/day6.txt");
  // read all the lines
  while (getline(dataFile, s)) {
    int distinctSequence = getDistinctSequence(s, 4);
    cout << "Part 1 : " << distinctSequence << "\n";

    distinctSequence = getDistinctSequence(s, 14);
    cout << "Part 2 : " << distinctSequence << "\n";
  }
}
