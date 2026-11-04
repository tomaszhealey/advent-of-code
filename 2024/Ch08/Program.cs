public class Program
{
    public static void Main(string[] args)
    {
        List<string> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").ToList();
        }

        new P1().Run(content);
        new P2().Run(content);
    }
}
public struct Location
{
    public int x;
    public int y;

    public Location(int x, int y)
    {
        this.x = x;
        this.y = y;
    }
}
