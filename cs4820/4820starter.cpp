#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <algorithm>
using namespace std;
// resident preferences
int resident_prefs[2000][2000];
// hospital preferences
int hosp_prefs[2000][2000];
// final_hosp_matches[i] will contain the resident hospital i is finally matched to
int final_hosp_matches[2000];
int main()
{
    int n;
    cin >> n;
    // reading in hospital preferences
    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            int next_pref;
            scanf("%i", &next_pref);
            hosp_prefs[i][j] = next_pref;
        }
    }