public class P2
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
        foreach (var item in updates)
        {
            var line = item;
            var it = 0;
            breakFlag = false;
            do
            {
                line = CheckList(orderings, ref breakFlag, line);
                it++;
            }
            while (breakFlag);

            if (it > 1)
                total += int.Parse(line[(line.Count + 1) / 2 - 1]);
        }

        Console.WriteLine("Part 2: " + total);

        static List<string> CheckList(Dictionary<string, List<string>> orderings, ref bool breakFlag, List<string> line)
        {
            breakFlag = false;
            List<string> valuesToCheck;
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
                        line.Remove(num);
                        line.Insert(i, num);
                        break;
                    }
                }
                if (breakFlag)
                    break;
            }

            return line;
        }
    }
}