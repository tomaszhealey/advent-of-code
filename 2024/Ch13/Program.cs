using Accord.Math;
using System.Diagnostics;

List<string> content;
using (var reader = new System.IO.StreamReader("input.txt"))
{
    content = reader.ReadToEnd().Split("\r\n\r\n").ToList();
}

Dictionary<string, Dictionary<string, decimal>> FormatOffset(string[] x, decimal offset)
{
    var dict = new Dictionary<string, Dictionary<string, decimal>>();

    dict.Add("A", new Dictionary<string, decimal>() { { "X", decimal.Parse(x[0].Substring(12, 2))}, { "Y", decimal.Parse(x[0].Substring(18, 2))} });
    dict.Add("B", new Dictionary<string, decimal>() { { "X", decimal.Parse(x[1].Substring(12, 2))}, { "Y", decimal.Parse(x[1].Substring(18, 2))} });
    var prize = x[2].Split(",").Select(y => y.Split("=")).ToList();
    dict.Add("Prize", new Dictionary<string, decimal>() { { "X", (decimal.Parse(prize[0][^1]) + offset)}, { "Y", (decimal.Parse(prize[1][^1]) + offset)} });

    return dict;
}

var watch = new Stopwatch();
watch.Start();
Console.WriteLine($"Part 1: {Part1(content.Select(x => FormatOffset(x.Split("\r\n"), 0)).ToList())}, {watch.ElapsedMilliseconds}ms");
watch.Restart();
Console.WriteLine($"Part 2: {Part2(content.Select(x => FormatOffset(x.Split("\r\n"), 10000000000000)).ToList())}, {watch.ElapsedMilliseconds}ms");



int Part1(List<Dictionary<string, Dictionary<string, decimal>>> content)
{
    var total = 0;

    foreach (var machine in content)
    {
        var max = machine["Prize"]["X"] / machine["B"]["X"];
        for (var b = 0; b < max; b++)
        {
            var toMove = machine["Prize"]["X"] - (machine["B"]["X"] * b);
            if (toMove % machine["A"]["X"] != 0)
                continue;
            var As = (decimal)toMove / (decimal)machine["A"]["X"];
            if (As * machine["A"]["Y"] + b * machine["B"]["Y"] == machine["Prize"]["Y"])
            {
                total += ((int)As * 3) + b;
                break;
            }
        }
    }

    watch.Stop();
    return total;
}

long Part2(List<Dictionary<string, Dictionary<string, decimal>>> content)
{
    //I really dislike floating point errors
    var total = 0L;
    foreach (var machine in content)
    {
        var machineMatrix = Matrix.Create(new decimal[,]
        {
            { machine["A"]["X"], machine["B"]["X"]},
            { machine["A"]["Y"], machine["B"]["Y"]},
        });
        var prizeMatrix = Matrix.Create(new decimal[,]
        {
            { machine["Prize"]["X"]},
            { machine["Prize"]["Y"]}
        });
        var abMatrix = machineMatrix.Inverse().Dot(prizeMatrix);
        //trying to account for floating point errors
        var a = Math.Round(abMatrix[0, 0], 8, MidpointRounding.AwayFromZero);
        var b = Math.Round(abMatrix[1, 0], 8, MidpointRounding.AwayFromZero);
        if (a == Math.Round(abMatrix[0, 0], 0, MidpointRounding.AwayFromZero) && b == Math.Round(abMatrix[1, 0], 0, MidpointRounding.AwayFromZero))
            total += (long)(3 * a) + (long)b;
    }

    watch.Stop();
    return total;
}