#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>
#include <sstream>
#include <iterator>
#include <algorithm>
#include <optional>

using namespace std;

struct Movies {
    string title;
    unordered_set<string> cast;
};

enum class Player {
    Veronique,
    Mark
};

// Helper function for reading lines from standard input
string read_line() {
    string line;
    getline(cin, line);
    return line;
}

// Splits a string into tokens
vector<string> split(const string& str) {
    istringstream iss(str);
    return vector<string>(istream_iterator<string>{iss}, istream_iterator<string>());
}

// Game function
optional<Player> game(const vector<Movies>& movies, const vector<string>& actresses, const vector<string>& actors, Player player, const string& turn);

// Simulates a move
optional<Player> new_new_move(const vector<Movies>& movies, const vector<string>& possible_actors, const vector<string>& possible_actresses, Player player, const string& turn) {
    if (player == Player::Veronique) {
        for (const auto& y : possible_actresses) {
            vector<const Movies*> c;
            for (const auto& m : movies) {
                if (m.cast.count(y) > 0 && m.cast.count(turn) > 0) {
                    c.push_back(&m);
                }
            }
            if (!c.empty()) {
                vector<string> n;
                copy_if(possible_actresses.begin(), possible_actresses.end(), back_inserter(n),
                        [&y](const string& x) { return x != y; });
                return game(movies, n, possible_actors, Player::Mark, y);
            }else{
                return Player::Mark;
            }
        }
        return nullopt;
    } else {
        // Implement Mark's move logic here if needed
        for (const auto& x : possible_actors) {
            vector<const Movies*> c;
            for (const auto& m : movies) {
                if (m.cast.count(x) > 0 && m.cast.count(turn) > 0) {
                    c.push_back(&m);
                }
            }
            if (!c.empty()) {
                vector<string> n;
                copy_if(possible_actors.begin(), possible_actors.end(), back_inserter(n),
                        [&x](const string& g) { return g != x; });
                return game(movies, possible_actresses, n, Player::Veronique, x);
            }else{
                return Player::Veronique;
            }
        }
        return nullopt;
    }
}

// Main game logic
optional<Player> game(const vector<Movies>& movies, const vector<string>& actresses, const vector<string>& actors, Player player, const string& turn) {
    if (actresses.empty() && player == Player::Veronique) {
        return Player::Mark;
    } else if (actors.empty() && player == Player::Mark) {
        return Player::Veronique;
    } else if (player == Player::Mark) {
        return new_new_move(movies, actors, actresses, player, turn);
    } else {
        return new_new_move(movies, actors, actresses, player, turn);
    }
}

int main() {
    int n, m;
    cin >> n >> m;
    cin.ignore();

    vector<string> actresses(n);
    for (int i = 0; i < n; i++) {
        actresses[i] = read_line();
    }

    vector<string> actors(n);
    for (int i = 0; i < n; i++) {
        actors[i] = read_line();
    }

    vector<Movies> movie_casts;
    for (int i = 0; i < m; i++) {
        string movie_title = read_line();
        int cast_size;
        cin >> cast_size;
        cin.ignore();
        unordered_set<string> cast;
        for (int j = 0; j < cast_size; j++) {
            string actor_or_actress = read_line();
            cast.insert(actor_or_actress);
        }
        movie_casts.push_back({movie_title, cast});
    }

    if (!actresses.empty()) {
        string firstMove = actresses.front();
        vector<string> newList;
        copy_if(actresses.begin(), actresses.end(), back_inserter(newList),
                        [&firstMove](const string& g) { return g != firstMove; });
        optional<Player> result = game(movie_casts, newList, actors, Player::Mark, 
        firstMove);
        if (result.has_value()) {
            if (result.value() == Player::Veronique) {
                cout << "Veronique" << endl;
            } else {
                cout << "Mark" << endl;
            }
        } else {
            cout << "No winner." << endl;
        }
    }

    return 0;
}
