public class P1
{
    private int _total = 0;
    private int _width;
    private int _height;

    public P1(List<string> content)
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
                if (content[i][j] == 'X')
                {
                    CheckHorizontal(content, i, j);
                    CheckVertical(content, i, j);
                    CheckDiagonals(content, i, j);
                }
            }
        }
        Console.WriteLine("Part 1: " + _total);
    }

    private void CheckDiagonals(List<string> content, int i, int j)
    {
        //Left and up
        if (j > 2 && i > 2)
        {
            if (content[i - 1][j - 1] == 'M' && content[i - 2][j - 2] == 'A' && content[i - 3][j - 3] == 'S')
                _total++;
        }
        //Right and up
        if (j < _width - 3 && i > 2)
        {
            if (content[i - 1][j + 1] == 'M' && content[i - 2][j + 2] == 'A' && content[i - 3][j + 3] == 'S')
                _total++;
        }
        //Left and down
        if (j > 2 && i < _height - 3)
        {
            if (content[i + 1][j - 1] == 'M' && content[i + 2][j - 2] == 'A' && content[i + 3][j - 3] == 'S')
                _total++;
        }
        //Right and down
        if (j < _width - 3 && i < _height - 3)
        {
            if (content[i + 1][j + 1] == 'M' && content[i + 2][j + 2] == 'A' && content[i + 3][j + 3] == 'S')
                _total++;
        }
    }
    private void CheckHorizontal(List<string> content, int i, int j)
    {
        //right
        if (j > 2)
        {
            if (content[i][j - 1] == 'M' && content[i][j - 2] == 'A' && content[i][j - 3] == 'S')
                _total++;
        }
        //left
        if (j < _width - 3)
        {
            if (content[i][j + 1] == 'M' && content[i][j + 2] == 'A' && content[i][j + 3] == 'S')
                _total++;
        }
    }
    private void CheckVertical(List<string> content, int i, int j)
    {
        //up
        if (i > 2)
        {
            if (content[i - 1][j] == 'M' && content[i - 2][j] == 'A' && content[i - 3][j] == 'S')
                _total++;
        }
        //down
        if (i < _height - 3)
        {
            if (content[i + 1][j] == 'M' && content[i + 2][j] == 'A' && content[i + 3][j] == 'S')
                _total++;
        }
    }
}
