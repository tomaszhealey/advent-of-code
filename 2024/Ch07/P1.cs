using System.ComponentModel.DataAnnotations;
using System.Numerics;
using System.Reflection.Metadata.Ecma335;

public class P1
{
    public static void Run(List<string> content)
    {
        var lines = content.Select(x => x.Split(" ").ToList())
            .Select(x => RemoveColon(x))
            .Select(x => x
                .Select(y => long.Parse(y))
                .ToList())
            .ToList();

        var total = 0L;

        foreach (var line in lines)
        {
            var answer = line[0];
            line.RemoveAt(0);
            var maxBranches = line.Count - 1;
            var ops = new Func<long, long, long>[]
            {
                Sum,
                Multiply
            };

            var permutations = (int)Math.Pow(2, maxBranches);
            var middle = new List<int>();
            var branches = 1;
            var nums = Enumerable.Repeat((long)line[0], permutations).ToArray();

            for (int i = 0; i < maxBranches; i++)
            {
                var split = (int)(permutations / Math.Pow(2, branches));
                var op = 0;

                var chunks = nums.Chunk(split).ToArray();
                foreach (var chunk in chunks)
                {
                    for (int j = 0; j < split; j++)
                        chunk[j] = ops[op](chunk[j], line[branches]);
                    op = (op + 1) % 2;
                }
                nums = chunks.SelectMany(subArray => subArray).ToArray();
                branches++;
            }

            if (nums.Contains(answer))
            {
                total += answer;
            }
        }

        Console.WriteLine("Part 1: " + total);
    }
    private static long Multiply(long x, long y) => x * y;
    private static long Sum(long x, long y) => x + y;



    public static List<string> RemoveColon(List<string> a)
    {
        a[0] = a[0].Substring(0, a[0].Length - 1);
        return a;
    } 
}

