public class P2
{
    private int _total = 0;
    private int _width;
    private int _height;

    public P2(List<string> content)
    {
        _height = content.Count;
        _width = content[0].Length;
        Run(content);
    }

    public void Run(List<string> content)
    {
        for (int i = 0; i < _height; i++)
        {
            for (int j = 0; j < _width; j++)
            {
                if (content[i][j] == 'M')
                {
                    CheckFromTopLeft(content, i, j);
                    CheckFromBottomRight(content, i, j);
                }
            }
        }
        Console.WriteLine("Part 1: " + _total);
    }

    private void CheckFromBottomRight(List<string> content, int i, int j)
    {
        if (i > 1 && j > 1)
        {
            //2M on right
            if (content[i - 2][j] == 'M' && content[i - 1][j - 1] == 'A' && content[i][j - 2] == 'S' && content[i - 2][j - 2] == 'S')
                _total++;
            //2M on bottom
            if (content[i][j - 2] == 'M' && content[i - 1][j - 1] == 'A' && content[i - 2][j] == 'S' && content[i - 2][j - 2] == 'S')
                _total++;
        }
    }

    private void CheckFromTopLeft(List<string> content, int i, int j)
    {
        if (j < _width - 2 && i < _height - 2)
        {
            //2M on left
            if (content[i + 2][j] == 'M' && content[i + 1][j + 1] == 'A' && content[i][j + 2] == 'S' && content[i + 2][j + 2] == 'S')
                _total++;
            //2M on top
            if (content[i][j + 2] == 'M' && content[i + 1][j + 1] == 'A' && content[i + 2][j] == 'S' && content[i + 2][j + 2] == 'S')
                _total++;
        }
    }
}