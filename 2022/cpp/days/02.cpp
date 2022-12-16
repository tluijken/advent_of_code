#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char *argv[]) {
  string s;
  ifstream dataFile("../../rust/input/2022/day2.txt");
  vector<string> lines;

  // read all the lines
  while (getline(dataFile, s)) {
    lines.push_back(s);
  }

  // part 1
  int score = 0;
  for (string &line : lines) {
    int delta = (line[2] - line[0] - 23 + 3) % 3;
    score += line[2] - 'X' + 1 + (delta == 1 ? 6 : delta == 0 ? 3 : 0);
  }
  cout << "Part 1 : " << score << '\n';
  // part 2
  score = 0;
  for (string &line : lines) {
    score += line[2] == 'X'   ? (line[0] - 'A' - 1 + 3) % 3 + 1
             : line[2] == 'Y' ? 3 + line[0] - 'A' + 1
                              : 6 + (line[0] - 'A' + 1) % 3 + 1;
  }

  cout << "Part 2 : " << score << '\n';
  return 0;
}
