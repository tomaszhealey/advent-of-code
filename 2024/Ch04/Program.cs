public class Program
{
    public static void Main(string[] args)
    {
        List<string> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").ToList();
        }

        var p1 = new P1(content);
        var p2 = new P2(content);
    }
}
