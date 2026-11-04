using System.Dynamic;

public class P1
{
    public static void Run(List<char> content)
    {
        var numOfIds = 0;
        var expandedInput = new List<int?>();
        var isFile = true;
        var id = 0;
        foreach (var c in content)
        {
            int? toAdd = null;
            int spaces = c - '0';
            if (isFile)
            {
                toAdd = id;
                id++;
                numOfIds += spaces;
            }
            for (var i = 0; i < spaces; i++)
                expandedInput.Add(toAdd);
            isFile = !isFile;
        }

        var nextFree = expandedInput.IndexOf(null);

        for(int i = expandedInput.Count - 1; i >= 0; i--)
        {
            if (expandedInput[i] == null)
                continue;
            if (!expandedInput.Chunk(numOfIds).First().Aggregate(true, (a, c) => (a && c == null) ? false : true))
                break;
            var toMove = expandedInput[i];
            expandedInput[nextFree] = toMove;
            nextFree = expandedInput.IndexOf(null);
            expandedInput.RemoveAt(i);
        }

        expandedInput.RemoveAll(x => x == null);

        long? total = 0;
        for (int i = 0; i < expandedInput.Count; i++)
        {
            total += (long?) (expandedInput[i] * i);
        }
        Console.WriteLine("Part 1: " + total);
    }
}
