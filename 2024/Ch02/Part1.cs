using System;
using System.Collections.Generic;
using System.Linq;
using System.Security;
using System.Text;
using System.Threading.Tasks;

namespace Ch2
{
    public class Part1
    {
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

            var total = 0;
            bool toAdd;
            int multiplier;
            int difference;

            foreach (var line in content)
            {
                toAdd = true;
                //if it is descending multipy the difference by -1 so that it is positive. Means that i only need one if for both
                //ascending and descending records
                multiplier = (line[1] - line[0] < 0) ? -1 : 1;

                for (int i = 1; i < line.Count; i++)
                {
                    difference = multiplier * (line[i] - line[i - 1]);
                    if (difference <= 0 || difference > 3)
                    {
                        toAdd = false;
                        break;
                    }
                }
                if (toAdd)
                    total++;
            }

            Console.WriteLine(total);

        }
    }
}
