using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Ch2
{
    public class Part2
    {
        static int total = 0;

        public static void Run()
        {
            List<List<int>> content;
            using (var reader = new System.IO.StreamReader("input.txt"))
            {
                content = reader.ReadToEnd().Split("\r\n")
                    .Select(x => x.Split(" "))
                        .Select(y => y.Select(z => int.Parse(z)).ToList())
                    .ToList();
            }

            foreach (var line in content)
            {
                CheckLine(line);
            }

            Console.WriteLine(total);
        }

        public static void CheckLine(List<int> line)
        {
            var toAdd = true;
            int difference;

            //if it is descending multipy the difference by -1 so that it is positive. Means that i only need one if for both
            //ascending and descending records
            var multiplier = (line[1] - line[0] < 0) ? -1 : 1;

            for (int i = 1; i < line.Count; i++)
            {
                difference = multiplier * (line[i] - line[i - 1]);
                if (difference <= 0 || difference > 3)
                {
                    //brute force solution, trying to see if taking out one of the two erroneous values meant that
                    //some records were false flagged
                    toAdd = false;
                    for (int j = 0; j < line.Count; j++)
                        toAdd = (CheckAlteredLines(new List<int>(line), j)) ? true : toAdd;
                    break;
                }
            }
            if (toAdd)
                total++;
        }

        public static bool CheckAlteredLines(List<int> line, int removeAt)
        {
            line.RemoveAt(removeAt);
            int difference;
            var multiplier = (line[1] - line[0] < 0) ? -1 : 1;

            for (int i = 1; i < line.Count; i++)
            {
                difference = multiplier * (line[i] - line[i - 1]);
                if (difference <= 0 || difference > 3)
                {
                    return false;
                }
            }
            return true;
        }

    }
}
