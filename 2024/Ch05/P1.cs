public class P1
{
    public static void Run(List<string> content)
    {
        var pairs = content[0].Split("\r\n").ToList();
        var orderings = new Dictionary<string, List<string>>();

        foreach (var item in pairs)
        {
            var pair = item.Split('|');
            if (orderings.ContainsKey(pair[1]))
                orderings[pair[1]].Add(pair[0]);
            else
                orderings.Add(pair[1], new List<string>() { pair[0] });
        }

        int total = 0;
        bool breakFlag;
        List<string> valuesToCheck;
        var updates = content[1].Split("\r\n").Select(x => x.Split(",").ToList()).ToList();
        foreach (var line in updates)
        {
            breakFlag = false;
            for (int i = 0; i < line.Count; i++)
            {
                if (!orderings.ContainsKey(line[i]))
                    continue;

                valuesToCheck = line.Take(i).ToList();
                foreach (var num in orderings[line[i]])
                {
                    if (!line.Contains(num))
                        continue;
                    if (!valuesToCheck.Contains(num))
                    {
                        breakFlag = true;
                        break;
                    }
                }
                if (breakFlag)
                    break;
            }
            if (!breakFlag)
                total += int.Parse(line[(line.Count + 1) / 2 - 1]);
        }

        Console.WriteLine("Part 1: " + total);
    }
}
