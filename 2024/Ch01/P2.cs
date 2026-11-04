public class P2
{
    public static void Part2()
    {
        List<string> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").ToList();
        }

        var leftIDs = new List<int>();
        var rightIDs = new List<int>();
        var rightIDCount = new Dictionary<int, int>();

        List<string> pair;
        foreach (var line in content)
        {
            pair = line.Split("   ").ToList();
            leftIDs.Add(int.Parse(pair[0]));
            rightIDs.Add(int.Parse(pair[1]));
        }

        leftIDs.Sort();
        foreach (var id in rightIDs)
        {
            if (!rightIDCount.ContainsKey(id))
                rightIDCount.Add(id, 0);
            rightIDCount[id]++;
        }

        var total = 0;
        foreach (var id in leftIDs)
        {
            if (!rightIDCount.ContainsKey(id))
            {
                continue;
            }
            total += (id * rightIDCount[id]);
        }

        Console.WriteLine(total);
    }
}