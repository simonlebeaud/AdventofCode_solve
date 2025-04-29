#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <cmath>

using namespace std;

// Check if a sequence is safe (all increasing or all decreasing, and each step is 1-3)
bool isSafe(const vector<int>& levels) {
    if (levels.size() < 2) return false;
    bool increasing = true, decreasing = true;
    for (size_t i = 1; i < levels.size(); ++i) {
        int diff = levels[i] - levels[i-1];
        if (abs(diff) < 1 || abs(diff) > 3) return false;
        if (diff > 0) decreasing = false;
        if (diff < 0) increasing = false;
    }
    return increasing || decreasing;
}

// Check if removing one element makes the sequence safe
bool isSafeWithOneRemoval(const vector<int>& levels) {
    for (size_t i = 0; i < levels.size(); ++i) {
        vector<int> temp;
        for (size_t j = 0; j < levels.size(); ++j) {
            if (j != i) temp.push_back(levels[j]);
        }
        if (isSafe(temp)) return true;
    }
    return false;
}

int main() {
    ifstream infile("inputs.txt"); // Change filename if needed
    string line;
    int safe_count_part1 = 0, safe_count_part2 = 0;

    while (getline(infile, line)) {
        istringstream iss(line);
        vector<int> levels;
        int num;
        while (iss >> num) levels.push_back(num);

        if (isSafe(levels)) {
            ++safe_count_part1;
            ++safe_count_part2;
        } else if (isSafeWithOneRemoval(levels)) {
            ++safe_count_part2;
        }
    }

    cout << "Part 1: " << safe_count_part1 << endl;
    cout << "Part 2: " << safe_count_part2 << endl;
    return 0;
}