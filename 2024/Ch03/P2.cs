public class P2
{
    private static bool _enabled = true;
    public static void Run(List<string> input)
    {
        int total = 0;
        int sublength;

        foreach (var line in input)
        {
            for (int i = 0; i < line.Length; i++)
            {
                UpdateFlag(line, i);
                if (!_enabled)
                    continue;

                //I had to use a different method to identify mul functions since i think that my flag was not working correctly
                //I tried to find the issue but i gave up and ended up starting from scratch on a different solution, which worked
                if (i < line.Length - 3 &&
                    line[i] == 'm' && line[i + 1] == 'u' && line[i + 2] == 'l' && line[i + 3] == '(')
                {
                    sublength = (line.Length - 1 - i < 12) ? line.Length - 1 - i - 4 : 8;
                    var function = line.Substring(i + 4, sublength).Split(")")[0].Split(",");

                    int a; int b;
                    if (int.TryParse(function[0], out a) &&
                        int.TryParse(function[1], out b))
                    {
                        //check for 3 digit numbers
                        if (a < 1000 && b < 1000)
                            total += a * b;
                    }
                }
            }

        }
        Console.WriteLine("Part 2: " + total);
    }

    private static void UpdateFlag(string item, int i)
    {
        if (i <= item.Length - 3 &&
            item[i] == 'd' && item[i + 1] == 'o' && item[i + 2] == '(' && item[i + 3] == ')')
            _enabled = true;
        else if (i <= item.Length - 6 &&
            (item[i] == 'd' && item[i + 1] == 'o' && item[i + 2] == 'n' && item[i + 3] == '\'' && item[i + 4] == 't' && item[i + 5] == '(' && item[i + 6] == ')'))
        {
            _enabled = false;
        }
    }
}