public class P1
{
    public static void Part1()
    {
        List<string> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").ToList();
        }

        var leftIDs = new List<int>();
        var rightIDs = new List<int>();

        List<string> pair;
        foreach (var line in content)
        {
            pair = line.Split("   ").ToList();
            leftIDs.Add(int.Parse(pair[0]));
            rightIDs.Add(int.Parse(pair[1]));
        }

        leftIDs.Sort(); rightIDs.Sort();
        var total = 0;
        for (int i = 0; i < content.Count; i++)
        {
            if (leftIDs[i] > rightIDs[i])
                total += (leftIDs[i] - rightIDs[i]);
            else
                total += (rightIDs[i] - leftIDs[i]);
        }

        Console.WriteLine(total);
    }
}
