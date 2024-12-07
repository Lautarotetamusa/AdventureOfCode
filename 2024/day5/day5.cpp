#include <bits/stdc++.h>
#define forr(i, a, b) for (int i = (a); i < (b); i++)
#define forn(i, n) forr(i, 0, n)
#define dforn(i, n) for (int i = (n) - 1; i >= 0; i--)
#define forall(it, v) for (auto it = v.begin(); it != v.end(); it++)

using namespace std;

const int MAXN=100;
int N;
vector<int> G[MAXN];

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

void solve(){
    int Q;
    cin >> Q;
    int valid_sum = 0, invalid_sum = 0;
    while (Q--){
        string update_str; cin >> update_str;
        vector<int> update;
        vector<int> update_idx(MAXN, MAXN);
        int i = 0;
        for (string sub : split(update_str, ",")){
            int n = stoi(sub);
            update.push_back(n);
            update_idx[n] = i;
            i++;
        }

        bool valid_update = true;
        map<int, pair<int, int>> inout; // number: (in, out)

        forn(i, update.size()){
            int n = update[i];
            for (auto adj : G[n]) if (update_idx[adj] != MAXN) {
                valid_update &= update_idx[adj] >= i;
                inout[adj].first++; //in
                inout[n].second++; //out
            }
        }
        cout << (valid_update ? "valid" : "invalid") << endl;

        if (valid_update){
            valid_sum += update[update.size() / 2];
            continue;
        }
        // Sort first by in decreasing and then by out increasing
        vector<pair<int, pair<int, int>>> A; 
        for (auto& it : inout) { 
            A.push_back(it); 
        } 

        sort(A.begin(), A.end(), [](pair<int, pair<int, int>> &a, pair<int, pair<int, int>> &b) {
            pair<int, int> valueA = a.second;
            pair<int, int> valueB = b.second;
            if (valueA.first > valueB.first) return valueA.first > valueB.first;
            return valueA.second <= valueB.second;
        });

        invalid_sum += A[update.size() / 2].first;
        for (auto a : A){
            cout << a.first << " ";
        } cout << endl;
        for (auto a : A){
            cout << a.second.first << " ";
        } cout << endl;
        for (auto a : A){
            cout << a.second.second << " ";
        } cout << endl;

        cout << endl;
    }
    cout << "valid sum: " << valid_sum << endl;
    cout << "invalid sum: " << invalid_sum << endl;
}

int main() {
    ios::sync_with_stdio(false); cin.tie(NULL); cout.tie(NULL);

    cin >> N;
    forn(i, N){
        int a, b; 
        cin >> a >> b;
        G[a].push_back(b);
    }

    solve();

    return 0;
}
