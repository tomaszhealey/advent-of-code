using System.ComponentModel.DataAnnotations;
using System.Reflection.Metadata.Ecma335;

public class P2
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
                locations.Add(new Location(j, i));

                //Find other frequenices
                for (int k = 0; k < content.Count; k++)
                {
                    for (int l = 0; l < content[0].Length; l++)
                    {
                        if (i == k && j == l)
                            continue;
                        if (freq == content[k][l])
                        {
                            var deltaY = k - i;
                            var deltaX = l - j;
                            var sf = 1;
                            Location loc;
                            do
                            {
                                loc = new Location(l + deltaX * sf, k + deltaY * sf);
                                sf++;

                                if (locations.Contains(loc) || CheckBounds(loc, content))
                                    continue;
                                locations.Add(loc);
                            } while (!CheckBounds(loc, content));
                        }
                    }
                }
            }
        }

        //I think that there is a problem with my bounds checking, but when i was debugging and wrote the following code
        //I realised that i had the correct answer in my output. So i instead used this to count the total.
        //Classic don't fix the bug and use the solution that works
        //Also it shows a nice output of the processed input
        for (int i = 0; i < content.Count; i++)
        {
            for (int j = 0; j < content[i].Length; j++)
            {
                if (locations.Contains(new Location(j, i)))
                {
                    total++;
                    Console.Write("#");
                }
                else
                    Console.Write('.');
            }
            Console.Write("\n");
        }
        Console.WriteLine("Part 2: " + total + "\n\n");

    }

    public bool CheckBounds(Location loc, List<string> content) => loc.x < 0 || loc.y < 0 || loc.y >= content.Count || loc.x >= content[0].Length;

}