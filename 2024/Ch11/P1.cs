public class P1
{
    private static int maxBlinks = 75;
    private static int _total = 0;
    private static object _lock = new object(); // For thread safety

    public async static void Run(List<int> content)
    {
        var watch = System.Diagnostics.Stopwatch.StartNew();
        var processed = 0;
        Parallel.For(0, content.Count, async i =>
        {
            await Blink((long)content[i], 1, content[i]);
            processed++;
            Console.WriteLine($"Processed: {processed}/{content.Count}");
        });

        watch.Stop();
        Console.WriteLine("Part 2: " + _total);
        Console.WriteLine($"{watch.ElapsedMilliseconds}ms");
    }

    static async Task Blink(long stone, int it, long parent)
    {
        string stoneString = $"{stone}";
        var total = 0;

        if (it != maxBlinks)
        { 
            if (stone == 0)
                Task.Run( () => Blink(1, it + 1, stone));
            else if (stoneString.Length % 2 == 0)
            {
                var length = stoneString.Length / 2;
                var task1 = Blink(long.Parse(stoneString.Substring(0, length)), it + 1, stone);
                var task2 = Blink(long.Parse(stoneString.Substring(length, length)), it + 1, stone);

                Task.WhenAll(task1, task2);
            }
            else
            {
                Blink(stone * 2024, it + 1, stone);
            }
        }
        else
        {
            lock (_lock)
            {
                if (stone == 0)
                    _total++;
                else if (stoneString.Length % 2 == 0)
                {
                    var length = stoneString.Length / 2;
                    _total++;
                    _total++;
                }
                else
                {
                    _total++;
                }
            }
        }
    }
}
