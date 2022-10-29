#include <limits.h>
#include <assert.h>
#include <iostream>
#include <queue>
#include <vector>
#include <unordered_map>
#include <tuple>
#include <unordered_set>
using namespace std;

int BFS(vector<vector<pair<int, int>>> &graph, int s, int t,
        vector<pair<int, int>> &prev, const vector<int> &rescap)
{
  vector<bool> visited = vector<bool>(graph.size(), false);
  vector<int> minima = vector<int>(graph.size(), 0);
  queue<int> q;
  q.push(s);
  minima[s] = INT_MAX;
  visited[0] = true;
  while (q.size() > 0)
  {
    int u = q.front();
    int f = minima[u];
    q.pop();
    for (pair<int, int> p : graph[u])
    {
      int v = p.first;
      int idx = p.second;
      if (!visited[v] && rescap[idx] > 0)
      {
        int new_f = min(f, rescap[idx]);
        q.push(v);
        minima[v] = new_f;
        visited[v] = true;
        prev[v] = {u, idx};
        if (v == t)
        {
          return minima[v];
        }
      }
    }
  }
  return 0;
}

void add_edge(tuple<int,int,int> from, tuple<int,int,int> to, 
  vector<int> eh, vector<int> et, vector<int> ec,
  unordered_map<tuple<int,int,int>,int> cn){
    if (cn.find(to) != cn.end()){
      int u_out = cn.find(from)->second + 1;
      int v_in = cn.find(to)->second;

      eh.push_back(u_out);
      et.push_back(v_in);
      ec.push_back(INT_MAX);
    }
  }

int main()
{
  int s = 0;
  int t = 1;
  int num_vertices = 0;
  int num_edges = 0;
  auto edge_heads = vector<int>();
  auto edge_tails = vector<int>();
  auto edge_caps = vector<int>();

  auto card_node = unordered_map<tuple<int, int, int>, int>();
  // card -> node number
  auto gadget_edge = unordered_map<int, int>();
  // node -> idx of the gadget edge in the graph vectors
  int nn = 2;
  auto possible_colors = unordered_set<int>();

  // sample parsing code
  int n = 0;
  int m = 0;
  int k = 0;
  assert(scanf("%d %d %d\n", &n, &m, &k) == 3);
  for (int i = 0; i < n; i++)
  {

    int x;
    int c;

    while (cin.peek() != '\n' && cin.peek() != -1)
    {
      assert(scanf("%d %d", &x, &c) == 2);
      fprintf(stderr, "Friend %d has card with value %d, color %d\n", i, x, c);

      possible_colors.insert(c);

      // construct all nodes, let them have the correct gadget and s, t connections
      int node = -1;
      auto search = card_node.find(make_tuple(i, x, c));
      if (search == card_node.end())
      {
        // make in node
        node = nn;
        card_node[make_tuple(i, x, c)] = nn;
        // make s-in edge if appropriate
        if (x == 1)
        {
          edge_heads.push_back(s);
          edge_tails.push_back(node);
          edge_caps.push_back(INT_MAX);
        }

        // make out (gadget) node
        gadget_edge[node] = edge_heads.size();
        edge_heads.push_back(nn);
        edge_tails.push_back(nn + 1);
        edge_caps.push_back(1);
        // make out-t edge if appropriate
        if (x == m)
        {
          edge_heads.push_back(node + 1);
          edge_tails.push_back(t);
          edge_caps.push_back(INT_MAX);
        }

        nn = nn + 2;
      }
      else
      { // the (i, x, c) was already seen
        node = search->second;
        auto e_s = gadget_edge.find(node);
        assert(e_s != gadget_edge.end());
        int edge = e_s->second;
        edge_caps[edge] = edge_caps[edge] + 1;
      }
    }
    cin.get();
  }

  for (auto kv: card_node){
    auto card = kv.first;
    int i = get<0>(card);
    int x = get<1>(card);
    int c = get<2>(card);

    auto next = make_tuple(i, x+1, c);
    add_edge(card, next, edge_heads, edge_tails, edge_caps, card_node);

    next = make_tuple(i+1, x+1, c);
    add_edge(card, next, edge_heads, edge_tails, edge_caps, card_node);

    for (auto color: possible_colors){
      if (color != c) {
        next = make_tuple(i, x, color);
        add_edge(card, next, edge_heads, edge_tails, edge_caps, card_node);
        next = make_tuple(i+1, x, color);
        add_edge(card, next, edge_heads, edge_tails, edge_caps, card_node);
      }
    }

  }

  // sample graph gen code
  // these are hard-coded into arrays only because it's easier to read off the original graph.
  // you'll probably want to generate the vectors on the fly in your implementation without intermediate arrays.
  // int edge_heads[] = { 0, 0, 2, 1, 2 };
  // int edge_tails[] = { 1, 2, 1, 3, 3 };
  // int edge_caps[] = { 1, 100, 1, 100, 1 };
  // int num_vertices = 4;
  // int num_edges = 5;

  vector<int> cap;
  auto graph = vector<vector<pair<int, int>>>(num_vertices, vector<pair<int, int>>());
  // int s = 0;
  // int t = 3;

  int edge_index = 0;
  for (int i = 0; i < num_edges; i++)
  {
    int u = edge_heads[i];
    int v = edge_tails[i];
    int c = edge_caps[i];

    // note that forwards edge is directly followed by backwards edge, this is required
    graph[u].push_back({v, edge_index++});
    graph[v].push_back({u, edge_index++});
    cap.push_back(c);
    cap.push_back(0);
  }

  int ans = 0;
  auto prev = vector<pair<int, int>>(graph.size(), {-1, -1});
  vector<int> rescap = vector<int>(cap);
  int f = BFS(graph, s, t, prev, rescap);
  int count = 0;

  while (f)
  {
    if (count % 100 == 0)
      fprintf(stderr, "[FF debug] iteration %d: bottleneck capacity %d\n", count++, f);
    else
      count++;
    int v = t;
    int sub_count = 1;
    while (v != s)
    {
      auto p = prev[v];
      int u = p.first;
      int idx = p.second;
      int rev_idx = (cap[idx] == 0) ? idx - 1 : idx + 1;
      rescap[idx] -= f;
      rescap[rev_idx] += f;
      v = u;
      sub_count++;
    }
    // fprintf(stderr, "%d\n", sub_count);
    ans += f;
    f = BFS(graph, s, t, prev, rescap);
  }
  printf("%d\n", ans);
}