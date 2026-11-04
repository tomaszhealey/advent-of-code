using System.Data;
using System.Diagnostics;
using System.Numerics;
using System.Reflection;
using System.Reflection.Metadata.Ecma335;

namespace Day_16
{
    internal class Program
    {
        static List<string> input;
        
        static void Main(string[] args)
        {
            using (var reader = new StreamReader("Input.txt"))
            {
                input = reader.ReadToEnd().ReplaceLineEndings().Split("\r\n").ToList();
            }
            var watch = new Stopwatch();

            CreateGraph(input, out (int, int) start, out (int, int) end);

            watch.Start();
            var output = Part1(start, end);
            watch.Stop();
            Console.WriteLine($"Part 1: {output}, Time: {watch.Elapsed.TotalMicroseconds} μs");

            watch.Restart();
            output = Part2(start, end, output);
            watch.Stop();
            Console.WriteLine($"Part 2: {output}, Time: {watch.Elapsed.TotalMicroseconds} μs");
        }

        static Dictionary<(int Row, int Col), List<(int Row, int Col)>> Graph = new Dictionary<(int Row, int Col), List<(int Row, int Col)>>();

        static long Part1((int Row, int Col) start, (int Row, int Col) end)
        {
            return Dijkstra(start, end);
        }
        static long Part2((int Row, int Col) start, (int Row, int Col) end, long shortest)
        {
            Memo = new List<(int Row, int Col)>() { {start} };
            foreach (var adj in Graph[end])
            {
                var dir = Directions.IndexOf((adj.Row - end.Row, adj.Col - end.Col));
                FindTiles(end, adj, shortest - 1, dir, start);
            }
            var i = input.Select(x => x.ToCharArray()).ToList();
            foreach (var pos in Memo)
                i[pos.Row][pos.Col] = 'O';
            Memo.Select(x => i[x.Row][x.Col] = 'O');
            i.ForEach(x => Console.WriteLine(new string(x)));
            return Memo.Count() + 1; //+1 for the end (where recursion starts from)
        }

        static List<(int Row, int Col)> Directions = new List<(int Row, int Col)>()
        {
            (-1, 0), //up
            (0, 1), //right
            (1, 0), //down
            (0, -1) //left
        };

        static void CreateGraph(List<string> input, out (int, int) start, out (int, int) end)
        {
            var map = input.Select(x => x.ToList()).ToList();
            start = end = (-1, -1);
            int nRow;
            int nCol;

            for (int row = 1; row < map.Count - 1; row++) 
            { 
                for (int col = 1; col < map[row].Count - 1; col++)
                {
                    if (input[row][col] == '#')
                        continue;

                    Graph.Add((row, col), new List<(int Row, int Col)>());
                    if (map[row][col] == 'S')
                        start = (row, col);
                    else if (map[row][col] == 'E')
                        end = (row, col);

                    foreach (var direction in Directions)
                    {
                        nRow = row + direction.Row;
                        nCol = col + direction.Col;
                        if (map[nRow][nCol] != '#')
                        {
                            Graph[(row, col)].Add((nRow, nCol));
                        }
                    }
                }
            }
        }

        public class Node
        {
            public int Dir = 1;
            public List<long> Costs = new List<long>() { long.MaxValue };

            //ofcourse, for a normal Dijkstra, you would update with a lowest but because of turns at junctions
            //sometimes the lowest at that moment will not be the lowest for the actual shortest path. This only matters for Part 2.
            public long Cost
            {
                get { return Costs.Min(); } 
            }

            public Node() { }
        }

        static Dictionary<(int Row, int Col), Node> DijkstraResult;

        static long Dijkstra((int Row, int Col) start, (int Row, int Col) end)
        {
            DijkstraResult = Graph.Keys.Select(x => (x, new Node())).ToDictionary();
            DijkstraResult[start].Costs.Add(0);
            var queue = new Queue<(int Row, int Col)>();
            int nDirection;
            long nCost;

            queue.Enqueue(start);

            while (queue.Count > 0)
            {
                var node = queue.Dequeue();

                foreach (var adjNode in Graph[node])
                {
                    nDirection = Directions.IndexOf((adjNode.Row - node.Row, adjNode.Col - node.Col));
                    if (DijkstraResult[node].Dir == nDirection)
                        nCost = DijkstraResult[node].Cost + 1;
                    else nCost = DijkstraResult[node].Cost + 1001;
                    var toTurn = input[adjNode.Row + Directions[nDirection].Row][adjNode.Col + Directions[nDirection].Col] == '#';

                    if (nCost + (toTurn ? 1000 : 0) < DijkstraResult[adjNode].Cost)
                    {
                        DijkstraResult[adjNode].Costs.Add(nCost);
                        DijkstraResult[adjNode].Dir = nDirection;
                        queue.Enqueue(adjNode);
                    }
                }
            }

            return DijkstraResult[end].Cost;
        }

        static List<(int Row, int Col)> Memo = new List<(int Row, int Col)>();

        static void FindTiles((int Row, int Col) previous, (int Row, int Col) current, long costLeft, int dir, (int Row, int Col) end)
        {
            if (costLeft < 1 || current == end)
                return;
            if (DijkstraResult[current].Cost == costLeft || DijkstraResult[current].Cost == costLeft - 1000 || DijkstraResult[current].Costs.Contains(costLeft))
                Memo.Add(current);
            else return;

            foreach (var adj in Graph[current])
            {
                if (previous == adj)
                    continue;

                var nDir = Directions.IndexOf((adj.Row - current.Row, adj.Col - current.Col));
                var nCost = costLeft - 1 - (nDir == dir ? 0 : 1000);

                if (Memo.Contains(adj))
                    continue;

                FindTiles(current, adj, nCost, nDir, end);
            }
        }
    }
}
