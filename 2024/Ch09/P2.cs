
public class P2
{
    public static void Run(List<char> content)
    {
        var length = 0;
        var expandedInput = new List<int[]>(); //negative keys are free spaces
        var isFile = true;
        var maxId = 0;

        foreach (var c in content)
        {
            int toAdd = maxId;
            int spaces = c - '0';
            if (isFile)
                maxId++;
            else
                toAdd = -1 * (maxId + 1);//+1 because start id is 0 and you cannot have -0. Also the id of null values does not matter only that it is less than 0.
            expandedInput.Add(new int[2] { toAdd, spaces });
            isFile = !isFile;
            length += spaces;
        }

        var nullId = -1;
        var checkedIds = new List<int>();
        for (int i = expandedInput.Count - 1; i >= 0; i--)
        {
            if (expandedInput[i][0] < 0 || checkedIds.Contains(expandedInput[i][0]))
                continue;
            var indexToInsertAt = FindNextFree(expandedInput, expandedInput[i][1], i);
            if (indexToInsertAt != -1)
            {
                checkedIds.Add(expandedInput[i][0]);
                expandedInput.Insert(indexToInsertAt, (int[]) expandedInput[i].Clone());
                var diff = expandedInput[indexToInsertAt + 1][1] - expandedInput[i + 1][1];
                expandedInput[i + 1][0] = nullId;
                if (diff != 0) 
                { 
                    expandedInput[indexToInsertAt + 1][1] = diff;
                    i++;
                }
                else
                    expandedInput.RemoveAt(indexToInsertAt + 1);
            }
        }

        long total = 0;
        int index = 0;
        foreach (var file in expandedInput)
        {
            if (file[0] >= 0)
            {
                for (int j = index; j < index + file[1]; j++)
                    total += (long)(file[0] * j);
            }
            index += file[1];
        }
        Console.WriteLine("Part 2: " + total);
    }

    private static int FindNextFree(List<int[]> list, int minSpaces, int maxIndex)
    {
        for (int i = 0; i < maxIndex; i++)
            if (list[i][0] < 0 && list[i][1] >= minSpaces) return i;
        return -1;
    }
}