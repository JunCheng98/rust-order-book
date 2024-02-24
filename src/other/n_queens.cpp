#include <ctime>
#include <cstdlib>
#include <iostream>
#include <unordered_map>

// r+c
std::unordered_map<int, int> posDiag{};
// r-c
std::unordered_map<int, int> negDiag{};
// r
std::unordered_map<int, int> row{};
// queen location in each column
std::unordered_map<int, int> colToQueenRow{};

void printBoard(std::vector<std::vector<int>>& board) {
    for (int i = 0; i < board.size(); i++) {
        for (int j = 0; j < board[i].size(); j++) {
            std::cout << board[i][j] << " ";
        }
        std::cout << '\n';
    }
}

std::vector<std::vector<int>> randomise(int n) {
    std::vector<std::vector<int>> board(n, std::vector<int>(n, 0));
    for (int i = 0; i < n; i++) {
        int r = std::rand() % n;
        board[r][i] = 1;
        colToQueenRow[i] = r;

        posDiag[r+i]++;
        negDiag[r-i]++;
        row[r]++;
    }
    return board;
}

int calculateAttacks() {
    int total = 0;
    // if there is only one, means no attacks along that diagonal/row, so we dont count it
    for (const auto& [_, num] : posDiag) {
        if (num == 0) continue;
        total += num-1;
    }
    for (const auto& [_, num] : negDiag) {
        if (num == 0) continue;
        total += num-1;
    }
    for (const auto& [_, num] : row) {
        if (num == 0) continue;
        total += num-1;
    }

    return total;
}

void moveQueen(std::vector<std::vector<int>>& board, int c) {
    int r = colToQueenRow[c];
    int newR = r;
    int currMin = posDiag[r+c] + negDiag[r-c] + row[r];
    // iterate all rows to find best position
    for (int i = 0; i < board.size(); i++) {
        if (currMin > posDiag[i+c] + negDiag[i-c] + row[i]) {
            currMin = posDiag[i+c] + negDiag[i-c] + row[i];
            newR = i;
        }
    }
    // already have the best queen position so far
    if (r == newR) return;

    // update all the data
    colToQueenRow[c] = newR;
    board[newR][c] = 1;
    posDiag[newR+c]++;
    negDiag[newR-c]++;
    row[newR]++;

    board[r][c] = 0;
    posDiag[r+c]--;
    negDiag[r-c]--;
    row[r]--;
}

void solve(int n) {
    if (n == 2 || n == 3) {
        std::cout << "problem is unsolvable with the given n value\n";
        return;
    }
    std::vector<std::vector<int>> board = randomise(n);

    std::cout << "Before: \n";
    printBoard(board);

    int attacks = calculateAttacks();
    // iterate until we find a solution with no attacks
    while (attacks > 0) {
        // select a random column
        int c = std::rand() % n;
        moveQueen(board, c);

        attacks = calculateAttacks();
    }

    std::cout << "After: \n";
    printBoard(board);
}

int main() {
    std::srand(std::time(nullptr));
    
    int n{};
    std::cout << "Enter n: ";
    std::cin >> n;

    solve(n);
    return 0; 
}