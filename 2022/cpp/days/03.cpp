#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

bool contains(string &input, char &chr) {
  return input.find(chr) != string::npos;
}

int getPriotity(char &chr) {
  int d = (int)chr;
  d -= d <= 90 ? 38 : 96;
  return d;
}

int main() {
  string s;
  ifstream dataFile("../../rust/input/2022/day3.txt");
  int priority = 0;
  vector<string> lines;

  // read all the lines
  while (getline(dataFile, s)) {
    lines.push_back(s);
  }

  // Part 1
  for (int i = 0; i < lines.size(); i++) {
    string s = lines[i];
    string firstHalf = s.substr(0, s.length() / 2);
    string secondHalf = s.substr(s.length() / 2, s.length());
    for (int i = 0; i < secondHalf.length(); i++) {
      if (contains(firstHalf, secondHalf[i])) {
        priority += getPriotity(secondHalf[i]);
        break;
      }
    }
  }
  cout << "Day 3 part 1 : " << priority << "\n";

  // Part 2
  priority = 0;
  for (int i = 0; i < lines.size(); i += 3) {
    for (int j = 0; j < lines[i].length(); j++) {
      if (contains(lines[i + 1], lines[i][j]) &&
          contains(lines[i + 2], lines[i][j])) {
        priority += getPriotity(lines[i][j]);
        break;
      }
    }
  }
  cout << "Day 3 part 2 : " << priority << "\n";

  return 0;
}
