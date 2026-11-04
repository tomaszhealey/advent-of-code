using System.Diagnostics;
public class Optimized
{
    public static int _maxBlinks = 75;
    static Dictionary<long, List<long>> cache = new Dictionary<long, List<long>>();

    //with help from robhabraken on github, almost all his code but i rewrote it in a way which i prefer
    public static void Run(List<int> content)
    {
        var watch = Stopwatch.StartNew();
        var stones = new Dictionary<long, long>();
        foreach (var stone in content)
            stones.Add(stone, 1);

        //Does each blink instead of each stone first
        //Why i could not do this in my previous solution is because i did each stone first so if i could not do
        //one operation for multiple stones since i didn't know how many stone there were in the same blink
        for (var i = 0; i < _maxBlinks; i++)
        {
            var blink = new Dictionary<long, long>();
            foreach (var stone in stones.Keys)
            {
                var multiplier = stones[stone];
                foreach (var newStone in UpdateStones(stone))
                    if (!blink.TryAdd(newStone, multiplier))
                        blink[newStone] += multiplier;
            }
            stones = blink;
        }

        watch.Stop();
        var total = 0L;
        foreach (var stone in stones.Keys)
            total += stones[stone];
        Console.WriteLine($"Optimized answer: {total}, {watch.ElapsedMilliseconds}ms");
    }

    private static List<long> UpdateStones(long number)
    {
        if (cache.TryGetValue(number, out List<long>? value))
            return value;

        var newStones = new List<long>();
        var stoneString = $"{number}";
        var length = stoneString.Length / 2;
        if (number == 0)
            newStones.Add(1);
        else if (stoneString.Length % 2 == 0)
        {
            newStones.Add(long.Parse(stoneString.Substring(0, length)));
            newStones.Add(long.Parse(stoneString.Substring(length, length)));
        }
        else
            newStones.Add(number * 2024);

        cache.Add(number, newStones);
        return newStones;
    }
}

