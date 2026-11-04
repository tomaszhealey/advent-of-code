public class Program
{
    public static void Main(string[] args)
    {
        List<char> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").ToList()[0].ToList();
        }

        //p2 is quicker than p1 for the first time ever, truly incredible.
        P1.Run(content);
        P2.Run(content);
    }
}
