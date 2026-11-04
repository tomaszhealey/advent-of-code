public class Program
{
    public static void Main(string[] args)
    {
        List<int[]> content;
        using (var reader = new System.IO.StreamReader("input.txt"))
        {
            content = reader.ReadToEnd().Split("\r\n").Select(x => x.Select(y => y - '0').ToArray()).ToList();
        }

        new P1(content).Run();
        new P2(content).Run();
    }
}
