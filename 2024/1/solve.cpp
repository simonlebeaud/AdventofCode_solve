#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_map>
#include <algorithm>

using namespace std;

int main() {
    std::ifstream input("inputs.txt");
    if (!input) {
        cerr << "Erreur ouverture fichier inputs.txt\n";
        return 1;
    }

    vector<int> leftList;
    vector<int> rightList;
    int leftVal, rightVal;

    // Lecture des deux listes
    while (input >> leftVal >> rightVal) {
        leftList.push_back(leftVal);
        rightList.push_back(rightVal);
    }

    if (leftList.size() != rightList.size()) {
        cerr << "Les listes n'ont pas la même taille\n";
        return 1;
    }

    // ---------- PARTIE 1 : Distance totale ----------
    vector<int> leftSorted = leftList;
    vector<int> rightSorted = rightList;

    sort(leftSorted.begin(), leftSorted.end());
    sort(rightSorted.begin(), rightSorted.end());

    long long totalDistance = 0;
    for (size_t i = 0; i < leftSorted.size(); ++i) {
        totalDistance += std::abs(leftSorted[i] - rightSorted[i]);
    }

    cout << "Distance totale (Partie 1) : " << totalDistance << endl;

    // ---------- PARTIE 2 : Similarity score ----------
    unordered_map<int, int> rightCount;
    for (int val : rightList) {
        rightCount[val]++;
    }

    long long similarityScore = 0;
    for (int val : leftList) {
        similarityScore += static_cast<long long>(val) * rightCount[val];
    }

    cout << "Similarity score (Partie 2) : " << similarityScore << endl;

    return 0;
}