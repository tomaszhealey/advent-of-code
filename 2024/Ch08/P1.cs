public class P1
{
    public void Run(List<string> content)
    {
        var total = 0;
        var locations = new List<Location>();
        for (int i = 0; i < content.Count; i++)
        {
            for (int j = 0; j < content[i].Length; j++)
            {
                if (content[i][j] == '.')
                    continue;

                var freq = content[i][j];

                //Find other frequenices
                for (int k = i + 1; k < content.Count; k++)
                {
                    for (int l = 0; l < content[0].Length; l++)
                    {
                        //if on the same line and the x is less than or equal to the x of the current frequency skip, avoids double
                        if (k == i && l <= j)
                            continue;
                        if (freq == content[k][l])
                        {
                            var deltaY = k - i;
                            var deltaX = l - j;

                            Location[] newLocations =
                            {
                                new Location(l + deltaX, k + deltaY),
                                new Location(j - deltaX, i - deltaY)
                            };

                            foreach (var loc in newLocations)
                            {
                                if (locations.Contains(loc) || loc.x < 0 || loc.y < 0 || loc.y >= content.Count || loc.x >= content[0].Length)
                                    continue;
                                locations.Add(loc);
                                total++;
                            }
                        }
                    }
                }
            }
        }
        Console.WriteLine("Part 1: " + total);
    }
}
