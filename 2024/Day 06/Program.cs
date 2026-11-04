using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

namespace Day_06
{
    internal class Program
    {
        static void Main(string[] args)
        {
            List<List<char>> input;
            var guardPosition = new Vector(0, 0);
            using (var reader = new StreamReader("Input.txt"))
            {
                input = reader.ReadToEnd().ReplaceLineEndings().Split("\r\n").Select(x => x.ToList()).ToList();

                for (int i = 0; i < input.Count; i++)
                {
                    if (input[i].Contains('^'))
                    {
                        guardPosition = new Vector(i, input[i].IndexOf('^'));
                        break;
                    }
                }
            }
            var watch = new Stopwatch();

            List<List<char>> patrolRoute;

            watch.Start();
            var output = Part1Ans = Part1(input, guardPosition.Clone(), out patrolRoute);
            watch.Stop();
            Console.WriteLine($"Part 1: {output}, Time: {watch.Elapsed.TotalMicroseconds} μs");

            watch.Restart();
            output = Part2(input, guardPosition.Clone(), patrolRoute);
            watch.Stop();
            Console.WriteLine($"Part 2: {output}, Time: {watch.Elapsed.TotalSeconds} s");
        }

        static long Part1Ans;

        static Vector[] Directions = new Vector[]
        {
            new Vector(-1, 0), //up
            new Vector(0, 1), //right
            new Vector(1, 0), //down
            new Vector(0, -1) //left
        };

        static long Part1(List<List<char>> input, Vector guardPosition, out List<List<char>> map)
        {
            map = input.Select(x => new List<char>(x)).ToList();
            int direction = 0;
            Vector nextPosition;
            var min = new Vector(0, 0);
            var max = new Vector(map.Count, map[0].Count);

            while (guardPosition.InRange(min, max))
            {
                map[guardPosition.Row][guardPosition.Col] = 'X';
                nextPosition = guardPosition + Directions[direction];
                try
                {
                    while (map[nextPosition.Row][nextPosition.Col] == '#')
                    {
                        direction = (direction + 1) % 4;
                        nextPosition = guardPosition + Directions[direction];
                    }
                    guardPosition = nextPosition;
                }
                catch (Exception)
                {
                    break;
                }
            }

            return map.Aggregate(0L, (a, x) => a + x.Aggregate(0L, (a1, y) => y == 'X' ? a1 + 1 : a1));
        }

        static List<(Vector, int)> Memo = new List<(Vector, int)>();

        static long Part2(List<List<char>> input, Vector guardPosition, List<List<char>> map)
        {
            var min = new Vector(0, 0);
            var max = new Vector(map.Count, map[0].Count);
            var output = 0L;
            var processed = 0;

            for (int col = 0; col < input.Count; col++)
            {
                for (int row = 0; row < input[0].Count; row++)
                {
                    if (map[row][col] == 'X')
                    {
                        Memo.Clear();
                        var newMap = input.Select(x => new List<char>(x)).ToList();
                        newMap[row][col] = '#';

                        output += FindLoop(newMap, guardPosition.Clone(), min, max);

                        processed++;
                        if (processed % 100 == 0)
                            Console.WriteLine($"Processed: {processed}/{Part1Ans}");
                    }
                }
            }

            return output;
        }

        //repurpose part 1
        static int FindLoop(List<List<char>> map, Vector guardPosition, Vector min, Vector max)
        {
            int direction = 0;
            Vector nextPosition;

            while (guardPosition.InRange(min, max))
            {
                if (Memo.Contains((guardPosition, direction)))
                    return 1;
                Memo.Add((guardPosition, direction));

                nextPosition = guardPosition + Directions[direction];
                try
                {
                    while (map[nextPosition.Row][nextPosition.Col] == '#')
                    {
                        direction = (direction + 1) % 4;
                        nextPosition = guardPosition + Directions[direction];
                    }
                    guardPosition = nextPosition;
                }
                catch (Exception)
                {
                    break;
                }
            }

            return 0;
        }
    }

    public class Vector
    {
        public int Row;
        public int Col;

        public Vector(int row, int col)
        {
            Row = row;
            Col = col;
        }

        public static Vector operator +(Vector a, Vector b)
        {
            return new Vector(a.Row + b.Row, a.Col + b.Col);
        }

        public override bool Equals(object? obj)
        {
            var a = obj as Vector;
            return Row == a.Row && Col == a.Col;
        }

        public Vector Clone()
        {
            return new Vector(Row, Col);
        }

        public bool InRange(Vector min, Vector max) => Row >= min.Row && Row <= max.Row && Col >= min.Col && Col <= max.Col;        
    }
}
