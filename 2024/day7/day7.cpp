#include <bits/stdc++.h>
#include <cmath>
#include <string>
#define forr(i, a, b) for (int i = (a); i < (b); i++)
#define forn(i, n) forr(i, 0, n)
#define dforn(i, n) for (int i = (n) - 1; i >= 0; i--)
#define forall(it, v) for (auto it = v.begin(); it != v.end(); it++)

using namespace std;

typedef unsigned long long ll;

std::vector<std::string> split(std::string s, std::string delimiter) {
    size_t pos_start = 0, pos_end, delim_len = delimiter.length();
    std::string token;
    std::vector<std::string> res;

    while ((pos_end = s.find(delimiter, pos_start)) != std::string::npos) {
        token = s.substr (pos_start, pos_end - pos_start);
        pos_start = pos_end + delim_len;
        res.push_back(token);
    }

    res.push_back(s.substr(pos_start));
    return res;
}

ll concat(ll a, ll b){
    ll log10 = b;
    ll exp = 1;
    while(log10){
        log10 /= 10;
        exp *= 10;
    }
    return a * exp + b;
}

bool find_permutation(vector<ll> &operators, int pos, ll result, ll acum) {
    if (pos >= operators.size()) return acum == result;

    return find_permutation(operators, pos+1, result, acum+operators[pos]) || 
           find_permutation(operators, pos+1, result, acum*operators[pos]) ||
           find_permutation(operators, pos+1, result, concat(acum, operators[pos]));
}

int main() {
    ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);

    int N;
    cin >> N;

    ll ans = 0;

    forn(i, N){
        string line;
        cin >> line;
        cout << line << endl;
        auto splitted = split(line, ",");
        ll result = stoll(splitted[0]); 
        cout << result << endl;
        vector<ll> operators;
        for(int i = 1; i < splitted.size(); i++){
            operators.push_back(stoi(splitted[i]));
        }

        bool can_be_true = find_permutation(operators, 1, result, operators[0]);
        if(can_be_true){
            ans += result;
        }
    }
    cout << "answer:" << ans << endl;

    return 0;
}
