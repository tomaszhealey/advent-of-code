public class Program
{
    public static void Main(string[] args)
    {
        List<int> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").First()
                    .Split(" ")
                    .Select(x => int.Parse(x))
                    .ToList();
        }

        //Simply change the "maxBlink" to change from p1 to p2
        Optimized.Run(content);
        P1.Run(content);
    }
}
