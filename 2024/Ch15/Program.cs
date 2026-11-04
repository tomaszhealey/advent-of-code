using System.Diagnostics;
using System.Numerics;

string[][] content;
var directions = new Dictionary<char, Vector2>()
{
    { '^', new Vector2(0, -1) },
    { '>', new Vector2(1, 0) },
    { '<', new Vector2(-1, 0) },
    { 'v', new Vector2(0, 1) },
};

using (var reader = new System.IO.StreamReader("input.txt"))
{
    content = reader.ReadToEnd().Split("\r\n\r\n").Select(x => x.Split("\r\n")).ToArray();
}

var instructions = string.Join(string.Empty, content[1]);
var originalMap = content[0].Select(x => x.ToArray()).ToArray();

var watch = new Stopwatch();
watch.Start();
Console.WriteLine($"Part 1: {Part1()}, {watch.ElapsedMilliseconds}ms");

watch.Restart();
Console.WriteLine($"Part 2: {Part2()}, {watch.ElapsedMilliseconds}ms");



int Part1()
{
    var map = originalMap.Select(x => x.ToList().ToArray()).ToArray();
    var robotPos = new Vector2();
    for (int i = 1; i < content[0].Length; i++)
    {
        robotPos.X = content[0][i].IndexOf('@');
        if (robotPos.X > 0)
        {
            robotPos.Y = i;
            break;
        }
    }
    map[(int)robotPos.Y][(int)robotPos.X] = '.';

    foreach ( var dir in instructions )
    {
        var nextTile = robotPos + directions[dir];
        var ntValue = GetValueAt(map, nextTile);

        if (ntValue == '#')
            continue;
        else if (ntValue == '.')
            robotPos = nextTile;

        var blockedByWall = false;
        var toMove = new List<Vector2>() { nextTile };
        while (ntValue == 'O')
        {
            nextTile += directions[dir];
            ntValue = GetValueAt(map, nextTile);
            toMove.Add(new Vector2(nextTile.X, nextTile.Y));
            if (ntValue == '#')
                blockedByWall = true;
        }
        if (blockedByWall)
            continue;
        robotPos = new Vector2(toMove[0].X, toMove[0].Y) ;
        map[(int)toMove[0].Y][(int)toMove[0].X] = '.';
        map[(int)toMove[^1].Y][(int)toMove[^1].X] = 'O';
        //odd issue where on position of robot a 0 spawns this fixes it but idk what causes it
        map[(int)robotPos.Y][(int)robotPos.X] = '.';
    }

    var total = 0;
    for (var i = 1; i < map.Length; i++)
    {
        for (var j = 1; j < map[0].Length; j++)
        {
            if (map[i][j] == 'O')
                total += (i * 100) + j;
        }
    }
    watch.Stop();
    return total;
}

int Part2()
{
    var tempMap = new List<List<char>>();
    var robotPos = new Vector2();
    int r = 0;

    foreach (var row in originalMap)
    {
        tempMap.Add(new List<char>());
        foreach (var col in row)
        {
            if (col == '@')
            {
                tempMap[r].Add(col); tempMap[r].Add('.');
            }
            else if (col == 'O')
            {
                tempMap[r].Add('['); tempMap[r].Add(']');
            }
            else
            {
                tempMap[r].Add(col); tempMap[r].Add(col);
            }
        }
        r++;
    }

    var map = tempMap.Select(x => x.ToArray()).ToArray();

    for (int i = 1; i < map.Length; i++)
    {
        for (int j = 2; j < map[0].Length; j++)
        {
            if (map[i][j] == '@')
                robotPos = new Vector2(j, i);
        }
    }
    AlterMap(map, robotPos, '.');


    foreach (var dir in instructions)
    {
        var nextTile = robotPos + directions[dir];
        var ntValue = GetValueAt(map, nextTile);

        if (ntValue == '#')
            continue;
        else if (ntValue == '.')
        {
            robotPos = nextTile;
            continue;
        }

        var blockedByWall = false;
        if (dir == '<' || dir == '>')
        {
            var toMove = new List<Vector2>() { nextTile };
            while ((ntValue == '[' || ntValue == ']') && !blockedByWall)
            {
                nextTile += directions[dir];
                ntValue = GetValueAt(map, nextTile);
                toMove.Add(new Vector2(nextTile.X, nextTile.Y));

                if (ntValue == '#')
                    blockedByWall = true;
                else if (ntValue == '.')
                    break;
            }
            if (blockedByWall) continue;

            robotPos = new Vector2(toMove[0].X, toMove[0].Y);
            map[(int)toMove[^1].Y][(int)toMove[^1].X] = map[(int)toMove[0].Y][(int)toMove[0].X];
            map[(int)toMove[0].Y][(int)toMove[0].X] = '.';
            map[(int)robotPos.Y][(int)robotPos.X] = '.';
            for (int i = 1; i < toMove.Count; i++)
            {
                var tile = toMove[i];
                map[(int)tile.Y][(int)tile.X] = Invert(map[(int)tile.Y][(int)tile.X]);
            }
        }
        else
        {
            var toMove = new List<List<Vector2>>() { new List<Vector2>() { robotPos } };
            var depth = 0;
            while (!blockedByWall)
            {
                if (toMove[depth].Count == 0)
                    break;
                toMove.Add(new List<Vector2>());
                foreach (var tile in toMove[depth])
                {
                    var boxTile = tile + directions[dir];
                    ntValue = GetValueAt(map, boxTile);
                    if (ntValue == '.' || toMove[depth + 1].Contains(boxTile))
                        continue;
                    else if (ntValue == '#')
                        blockedByWall = true;

                    toMove[depth + 1].Add(boxTile);

                    char boxDir = '*';
                    if (ntValue == '[')
                        boxDir = '>';
                    else
                        boxDir = '<';
                    toMove[depth + 1].Add(boxTile + directions[boxDir]);
                }
                depth++;
            }
            if (blockedByWall) continue;

            var newMap = map.Select(x => x.ToList().ToArray()).ToArray();
            for (int i = toMove.Count - 1; i > 0; i--)
            {
                foreach (var segment in toMove[i])
                {
                    AlterMap(newMap, segment, '.');
                    AlterMap(newMap, segment + directions[dir], GetValueAt(map, segment));
                }
            }
            robotPos += directions[dir];
            map = (char[][])newMap.Clone();
        }
    }
    var total = 0;
    for (var i = 1; i < map.Length; i++)
    {
        for (var j = 1; j < map[0].Length; j++)
        {
            if (map[i][j] == '[')
                total += (i * 100) + j;
        }
    }

    watch.Stop();
    return total;
}

char Invert(char c) => (c == '[') ? ']' : '[';

//useful for debugging or just pretty visuals
static void Showmap(char[][] map, Vector2 robotPos, char dir)
{
    Console.Clear();
    Console.WriteLine("Direction: " + dir);
    for (var i = 0; i < map.Length; i++)
    {
        for (var j = 0; j < map[0].Length; j++)
        {
            if (i == robotPos.Y && j == robotPos.X)
                Console.Write('@');
            else
                Console.Write(map[i][j]);
        }
        Console.WriteLine();
    }
    Console.ReadLine();
}

static char GetValueAt(char[][] map, Vector2 tile) => map[(int)tile.Y][(int)tile.X];
static void AlterMap(char[][] map, Vector2 tile, char c)
{
    map[(int)tile.Y][(int)tile.X] = c;
}