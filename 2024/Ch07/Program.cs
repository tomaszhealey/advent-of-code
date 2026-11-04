public class Program
{
    public static void Main(string[] args)
    {
        List<string> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").ToList();
        }

        P1.Run(content);
        P2.Run(content);
    }
}
