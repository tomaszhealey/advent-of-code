using System.Diagnostics;
using System.Numerics;

var axis = new List<int>()
{
    101, //width aka x
    103   //height aka y
};
List<Dictionary<string, Vector2>> robots;
using (var reader = new System.IO.StreamReader("input.txt"))
{
    robots = reader.ReadToEnd().Split("\r\n").ToList().Select(x => ParseLine(x)).ToList();
}

var watch = new Stopwatch();
watch.Start();
Console.WriteLine($"Part 1: {Part1(100)}, {watch.ElapsedTicks / TimeSpan.TicksPerMillisecond / 1000}µs");
watch.Restart();
Console.WriteLine($"Part 2: {Part2()}, {watch.ElapsedMilliseconds}ms");


int Part1(int elapsedSeconds)
{
    var quadrants = new List<int>() { 0, 0, 0, 0 }; //TL, TR, BL, BR
    var middleY = ((axis[1] + 1) / 2) - 1;
    var middleX = ((axis[0] + 1) / 2) - 1;

    foreach (var robot in robots)
    {
        var toMove = (robot["Position"] + (robot["Velocity"] * elapsedSeconds));
        toMove.X %= axis[0];
        toMove.Y %= axis[1];

        var finalPos = new Vector2();
        for (int i = 0; i < 2; i++)
            finalPos[i] = (toMove[i] < 0) ? toMove[i] + axis[i] : toMove[i];

        if (finalPos.X != middleX && finalPos.Y != middleY)
        {
            if (finalPos.Y < middleY)
            {
                if (finalPos.X < middleX)
                    quadrants[0]++;
                else
                    quadrants[1]++;
            }
            else
            {
                if (finalPos.X < middleX)
                    quadrants[2]++;
                else 
                    quadrants[3]++;
            }
        }
    }

    watch.Stop();
    return quadrants.Aggregate(1, (a, c) => a * c);
}

//check for a line of 8 consecutive bots since it will likely be the tree
int Part2()
{
    var total = 0;
    var board = new int[axis[1], axis[0]];
    var isChristmasTree = false;

    while (true)
    {
        ResetBoard(axis, board);
        total++;
        foreach (var robot in robots)
        {
            var toMove = (robot["Position"] + (robot["Velocity"] * total));
            toMove.X %= axis[0];
            toMove.Y %= axis[1];

            var finalPos = new Vector2();
            for (int i = 0; i < 2; i++)
                finalPos[i] = (toMove[i] < 0) ? toMove[i] + axis[i] : toMove[i];
            board[(int)finalPos.Y, (int)finalPos.X] = 1; ;
        }

        //Checks how many consecutive bots there are since it is likely to be a christmas tree if there are many in a row
        isChristmasTree = CheckForChristmasTree(axis, board, isChristmasTree);

        if (isChristmasTree)
            break;
    }

    //ShowBoard(axis, total, board);
    return total;
}

Dictionary<string, Vector2> ParseLine(string x)
{
    var split = x.Split(' ').Select(y => y.Split('=')[^1].Split(",")).ToList();
    return new Dictionary<string, Vector2>()
    {
        { "Position", new Vector2(float.Parse(split[0][0]), float.Parse(split[0][1]))},
        { "Velocity", new Vector2(float.Parse(split[1][0]), float.Parse(split[1][1]))}
    };
}

static void ResetBoard(List<int> axis, int[,] board)
{
    for (int i = 0; i < axis[1]; i++)
        for (int j = 0; j < axis[0]; j++)
            board[i, j] = 0;
}

//For coolnees value and debugging
static void ShowBoard(List<int> axis, int total, int[,] board)
{
    Console.WriteLine("----------------------\n Seconds Elapsed: " + total);
    for (int i = 0; i < axis[1]; i++)
    {
        for (int j = 0; j < axis[0]; j++)
            Console.Write((board[i, j] == 0) ? '.' : '#');
        Console.Write("\n");
    }
}

static bool CheckForChristmasTree(List<int> axis, int[,] board, bool isChristmasTree)
{
    var flag = false;
    for (int i = 0; i < axis[1]; i++)
    {
        var consecutive = 0;
        for (int j = 0; j < axis[0]; j++)
        {
            consecutive = (board[i, j] == 0) ? 0 : consecutive + 1;
            if (consecutive > 6)
            {
                isChristmasTree = true;
                flag = true;
                break;
            }
        }
        if (flag)
            break;
    }

    return isChristmasTree;
}