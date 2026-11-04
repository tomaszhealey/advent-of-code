public class P2
{
    private int _total = 0;
    private List<int[]> _content;

    public P2(List<int[]> content)
    {
        _content = content;
    }

    public void Run()
    {
        var trailheads = new List<(int, int, int)>();
        for (int i = 0; i < _content.Count; i++)
            for (int j = 0; j < _content[i].Length; j++)
                if (_content[i][j] == 0) trailheads.Add((i, j, 0));

        foreach (var trailhead in trailheads)
            CheckTrail(trailhead);

        Console.WriteLine("Part 2: " + _total);
    }

    private void CheckTrail((int, int, int) point)
    {
        var otherTrails = new List<(int, int, int)>(); //y, x, value
        if (point.Item1  != 0) //then check up
            AddTrailPoint(point.Item1 - 1, point.Item2, point.Item3, otherTrails);
        if (point.Item2 != 0)
            AddTrailPoint(point.Item1, point.Item2 - 1, point.Item3, otherTrails);
        if (point.Item1 < _content.Count - 1)
            AddTrailPoint(point.Item1 + 1, point.Item2, point.Item3, otherTrails);
        if (point.Item2 < _content[0].Length - 1)
            AddTrailPoint(point.Item1, point.Item2 + 1, point.Item3, otherTrails);

        foreach (var trail in otherTrails)
            CheckTrail(trail);
    }

    private void AddTrailPoint(int i, int j, int ogValue, List<(int, int, int)> trailList)
    {
        var value = _content[i][j];
        if (value - 1 == ogValue)
        {
            if (value == 9) 
                _total++;
            else 
                trailList.Add((i, j, value));
        }
    }
}
