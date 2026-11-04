using System.Reflection.Metadata.Ecma335;

public class P1
{
    public static void Run(List<string> input)
    {
        int total = 0;

        foreach (var line in input)
        {
            var content = line.Split("mul");
            foreach (var item in content)
            {
                if (item[0] != '(')
                    continue;

                var function = item.Split(")")[0].Split(",");

                if (function.Length == 1)
                    continue;

                int a; int b;
                if (int.TryParse(function[0].Substring(1, function[0].Length - 1), out a) &&
                    int.TryParse(function[1].Substring(0, function[1].Length), out b))
                {
                    //check for 3 digit numbers
                    if (a < 1000 && b < 1000)
                        total += a * b;
                }

            }
        }

        Console.WriteLine("Part 1: " + total);
    }
}