using System.ComponentModel;
using System.Diagnostics;
using System.Xml;

namespace Day_19
{
    internal class Program
    {
        static List<string> Towels;
        static void Main(string[] args)
        {
            List<string> combinations;
            using (var reader = new StreamReader("Input.txt"))
            {
                var input = reader.ReadToEnd().ReplaceLineEndings().Split("\r\n\r\n").ToList();
                Towels = input[0].Split(", ").ToList();
                combinations = input[1].Split("\r\n").ToList();
            }
            var watch = new Stopwatch();

            watch.Start();
            var output = Run(combinations);
            watch.Stop();
            Console.WriteLine($"Part 1: {output.Item1}\nPart 2: {output.Item2}" );
            Console.Write($"Time: {watch.Elapsed.TotalMilliseconds} ms");
        }

        static (int, long) Run(List<string> combos)
        {
            var possible = 0;
            var ways = 0L;
            foreach (var combo in combos)
            {
                Cache.Clear();
                var val = CheckIfPossible(combo);
                if (val > 0)
                {
                    possible++;
                    ways += val;
                }
            }

            return (possible, ways);
        }

        static Dictionary<string, long> Cache = new Dictionary<string, long>();
        private static long CheckIfPossible(string design)
        {
            if (design == "")
                return 1;

            var total = 0L;
            for (int i = 0; i < Towels.Count; i++)
            {
                if (Towels[i].Length > design.Length || !(design.Substring(0, Towels[i].Length) == Towels[i]))
                    continue;
                var nDesign = design.Substring(Towels[i].Length);

                if (Cache.TryGetValue(nDesign, out long value))
                    total += value;
                else total += CheckIfPossible(nDesign);
            }

            Cache.Add(design, total);
            return total;
        }
    }
}
