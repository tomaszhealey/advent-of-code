using System.Diagnostics;
using System.Dynamic;

namespace Day_18
{
    internal class Program
    {
        static (int Rows, int Colums) MapSize = (71, 71);
        static Dictionary<Vector, List<Vector>> Graph = new Dictionary<Vector, List<Vector>>();
        static Dictionary<Vector, List<Vector>> InitialGraph = new Dictionary<Vector, List<Vector>>();
        static Dictionary<Vector, int> Heuristics;

        static void Main(string[] args)
        {
            Vector start = new Vector(0, 0);
            Vector end = new Vector(MapSize.Rows - 1, MapSize.Colums - 1);

            CreateGraph(MapSize.Rows, MapSize.Colums);
            Heuristics = Graph.Select(x => (x.Key, (end.Row - x.Key.Row) + (end.Column - x.Key.Column))).ToDictionary();

            List<Vector> input;
            using (var reader = new StreamReader("Input.txt"))
            {
                input = reader.ReadToEnd().ReplaceLineEndings().Split("\r\n").Select(x => Vector.GetVector(x)).ToList();
            }
            var watch = new Stopwatch();

            InitialGraph = Graph.Select(x => (x.Key, new List<Vector>(x.Value))).ToDictionary();

            watch.Start();
            var output = Part1(input, 1024, start, end);
            watch.Stop();
            Console.WriteLine($"Part 1: {output}, Time: {watch.Elapsed.TotalMilliseconds} ms");

            watch.Restart();
            var output1 = Part2(input, start, end);
            watch.Stop();
            Console.WriteLine($"Part 2: {output1}, Time: {watch.Elapsed.TotalMilliseconds} ms");
        }

        static long Part1(List<Vector> input, int bytesToRead, Vector start, Vector end)
        {
            for (int i = 0; i < bytesToRead; i++)
            {
                var vector = input[i];
                Graph[vector].ForEach(adj => Graph[adj].Remove(vector));
                Graph.Remove(vector);
            }

            return AStar(start, end).Item2;
        }

        static string Part2(List<Vector> input, Vector start, Vector end)
        {
            var bytesToRead = input.Count / 2;
            var low = 0;
            var high = input.Count - 1;
            ResetGraph();
            DropBytes(input, 0, bytesToRead);

            while (low < high)
            {
                if (AStar(start, end).Item1)
                {
                    low = bytesToRead + 1;
                    bytesToRead = ((high - low) / 2) + low;
                    DropBytes(input, low, bytesToRead);
                }
                else
                {
                    high = bytesToRead;
                    bytesToRead = ((high - low) / 2) + low;
                    ResetGraph();
                    DropBytes(input, 0, bytesToRead);
                }
            }

            return input[bytesToRead - 1].ToString();
        }

        static void CreateGraph(int rows, int columns)
        {
            for (int row = 0; row < rows; row++)
            {
                for (int col = 0; col < columns; col++)
                {
                    var v = new Vector(row, col);
                    Graph[v] = new List<Vector>();
                    if (row > 0)
                        Graph[v].Add(new Vector(row - 1, col));
                    if (col > 0)
                        Graph[v].Add(new Vector(row, col - 1));
                    if (col < columns - 1)
                        Graph[v].Add(new Vector(row, col + 1));
                    if (row < rows - 1)
                        Graph[v].Add(new Vector(row + 1, col));
                }
            }
        }
        static void ResetGraph() => Graph = InitialGraph.Select(x => (x.Key, new List<Vector>(x.Value))).ToDictionary();

        static void DropBytes(List<Vector> input, int start, int bytesToRead)
        {
            for (int i = start; i < bytesToRead; i++)
            {
                var vector = input[i];
                Graph[vector].ForEach(adj => Graph[adj].Remove(vector));
                Graph.Remove(vector);
            }
        }

        static (bool, int) AStar (Vector start, Vector end)
        {
            var nodes = Graph.Select(x => (x.Key, int.MaxValue)).ToDictionary();
            var queue = new PriorityQueue<Vector, int>();
            nodes[start] = 0;
            queue.Enqueue(start, 0);

            Vector node;

            while (queue.Count > 0 && nodes[end] == int.MaxValue)
            {
                node = queue.Dequeue();
                foreach (var adj in Graph[node])
                {
                    var cost = nodes[node] + 1;
                    if (cost < nodes[adj])
                    {
                        nodes[adj] = cost;
                        queue.Enqueue(adj, cost + Heuristics[adj]);
                    }
                }
            }

            return (nodes[end] != int.MaxValue, nodes[end]);
        }
    }

    public class Vector
    {
        public int Row;
        public int Column;

        public Vector(int row, int column)
        {
            Row = row;
            Column = column;
        }

        public static Vector GetVector(string a)
        {
            var x = a.Split(',').Select(int.Parse).ToList();
            return new Vector(x[1], x[0]);
        }

        public override bool Equals(object? a)
        {
            var b = a as Vector;
            return Row == b.Row && Column == b.Column;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Row, Column);
        }

        public override string ToString()
        {
            return $"{Column},{Row}";
        }
    }
}
