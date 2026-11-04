using System.ComponentModel.Design;
using System.Diagnostics;

namespace Day_12
{
    internal class Program
    {
        static void Main(string[] args)
        {
            List<List<char>> input;
            using (var reader = new StreamReader("Input.txt"))
            {
                input = reader.ReadToEnd().ReplaceLineEndings().Split("\r\n").Select(x => x.ToList()).ToList();
            }

            var watch = new Stopwatch();

            watch.Start();
            var output = Part1(input);
            watch.Stop();
            Console.WriteLine($"Part 1: {output}, Time: {watch.Elapsed.TotalMicroseconds} μs");

            watch.Restart();
            output = Part2(input);
            watch.Stop();
            Console.WriteLine($"Part 2: {output}, Time: {watch.Elapsed.TotalMicroseconds} μs");
        }

        static bool[,] Memo;

        static long Part1(List<List<char>> input)
        {
            var output = 0L;

            Memo = new bool[input.Count, input[0].Count];

            for (int row  = 0; row < input.Count; row++)
            {
                for (int col = 0; col < input[row].Count; col++)
                {
                    if (!Memo[row, col])
                    {
                        (long perimiter, long area) = FloodPlots(input, row, col, input[row][col], (input.Count, input[0].Count));
                        output += perimiter * area;
                    }
                }
            }

            return output;
        }

        static List<(int Row, int Col)> Directions = new List<(int Row, int Col)>()
        {
            (-1, 0), //up
            (0, 1), //right
            (1, 0), //down
            (0, -1) //left
        };

        static (long, long) FloodPlots(List<List<char>> map, int row, int col, char target, (int, int) bounds)
        {
            Memo[row, col] = true;

            var perimiter = 4;
            var tPerimiter = 0L;
            var tArea = 0L;
            var direction = 0;
            int nRow;
            int nCol;

            for (direction =  0; direction < 4; direction++)
            {
                nRow = row + Directions[direction].Row;
                nCol = col + Directions[direction].Col;

                if (nRow < bounds.Item1 && nRow > -1 && nCol < bounds.Item2 && nCol > -1 && map[nRow][nCol] == target)
                {
                    perimiter--;
                    if (Memo[nRow, nCol])
                        continue;

                    var a = FloodPlots(map, row + Directions[direction].Row, col + Directions[direction].Col, target, bounds);
                    tPerimiter += a.Item1;
                    tArea += a.Item2;
                }
            }

            return (tPerimiter + perimiter, tArea + 1);

        }

        static PlotSideInfo[,] MemoP2;

        static long Part2(List<List<char>> input)
        {
            var output = 0L;
            MemoP2 = new PlotSideInfo[input.Count, input[0].Count];

            for (int row = 0; row < input.Count; row++)
            {
                for (int col = 0; col < input[row].Count; col++)
                {
                    if (MemoP2[row, col] == null)
                    {
                        var a = FloodPlotsP2(input, row, col, input[row][col], (input.Count, input[0].Count));
                        output += a.Item1 * a.Item2;
                    }
                }
            }

            return output;
        }

        static (long, long) FloodPlotsP2(List<List<char>> map, int row, int col, char target, (int, int) bounds)
        {
            var plotInfo = MemoP2[row, col] = new PlotSideInfo(target);

            var sides = 0;
            var tSides = 0L;
            var tArea = 0L;
            var side = 0;
            int nRow;
            int nCol;
            var directionsToCheck = new List<(int Row, int Col)>();

            //Update sides around current plot
            for (side = 0; side < 4; side++)
            {
                nRow = row + Directions[side].Row;
                nCol = col + Directions[side].Col;
                if (!(nRow < bounds.Item1 && nRow > -1 && nCol < bounds.Item2 && nCol > -1 && map[nRow][nCol] == target))
                {
                    switch (side)
                    {
                        case 0:
                            plotInfo.Top = true; break;
                        case 1:
                            plotInfo.Right = true; break;
                        case 2:
                            plotInfo.Bottom = true; break;
                        case 3:
                            plotInfo.Left = true; break;
                    }
                }
                else
                    directionsToCheck.Add((nRow, nCol));
            }

            plotInfo.SetSides();

            //process adjacent plots
            foreach (var next in directionsToCheck)
            {
                if (MemoP2[next.Row, next.Col] == null)
                {
                    var a = FloodPlotsP2(map, next.Row, next.Col, target, bounds);
                    tArea += a.Item2;
                    tSides += a.Item1;
                }
            }

            //check all sides and if they have already been counted
            bool toCount;
            for (side = 0; side < 4; side++)
            {
                if (plotInfo.Sides[side])
                {
                    toCount = true;
                    int perpDir = (side % 2) + 1;
                    for (int i = 0; i < 2; i++)
                    {
                        nRow = row + Directions[perpDir].Row;
                        nCol = col + Directions[perpDir].Col;
                        while (nRow > -1 && nRow < bounds.Item1 && nCol > -1 && nCol < bounds.Item2 && MemoP2[nRow, nCol] != null && MemoP2[nRow, nCol].Type == target && MemoP2[nRow, nCol].Sides[side])
                        {
                            if (plotInfo.CheckPlot(nRow, nCol, side, row, col))
                            {
                                toCount = false;
                                break;
                            }
                            nRow += Directions[perpDir].Row;
                            nCol += Directions[perpDir].Col;
                        }

                        if (!toCount)
                            break;

                        perpDir = (perpDir + 2) % 4;
                    }

                    if (toCount)
                        sides++;
                }
            }

            plotInfo.IsProcessed = true;
            return (tSides + sides, tArea + 1);
        }

        public class PlotSideInfo
        {
            public char Type;
            public bool IsProcessed = false;
            //Sides
            public bool Left = false;
            public bool Right = false;
            public bool Top = false;
            public bool Bottom = false;
            public bool[] Sides = new bool[4];

            public PlotSideInfo(char type)
            {
                Type = type;
            }

            public void SetSides()
            {
                Sides = new bool[] { Top, Right, Bottom, Left };
            }

            //returns true if the side being checked has been counted
            public bool CheckPlot(int nRow, int nCol, int side, int row, int col)
            {
                var adjPlot = MemoP2[nRow, nCol];
                if (!adjPlot.IsProcessed)
                    return false;
                return adjPlot.Sides[side];
            }
        }
    }
}
