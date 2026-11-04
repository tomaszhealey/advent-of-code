using System.Diagnostics;

namespace Day_17
{
    internal class Program
    {
        static long A;
        static long B;
        static long C;
        static int InstructionPntr;
        static List<int> Output = new List<int>();
        static bool ShowProgram = false; //used for understanding the input program, the A register is shown in octal not denary
        static long GetComboOperand(long operand)
        {
            switch (operand)
            {
                case 4:
                    return A;
                case 5:
                    return B;
                case 6:
                    return C;
                default:
                    return operand;
            }
        }

        static List<Action<int>> Operations = new List<Action<int>>
        {
            x => A = (long)(A / Math.Pow(2, GetComboOperand(x))),
            x => B ^= x,
            x => B = GetComboOperand(x) % 8,
            x => InstructionPntr = (A == 0) ? InstructionPntr : x,
            x => B ^= C,
            x => Output.Add((int)(GetComboOperand(x) % 8)),
            x => B = (long)(A / Math.Pow(2, GetComboOperand(x))),
            x => C = (long)(A / Math.Pow(2, GetComboOperand(x)))
        };

        //operand is placed after the explanation, LSF = least significant
        static List<string> InstructionExplanations = new List<string>()
        {
            "Ignore ComboOperand LSF bit(s) of A: ",
            "B ^= operand: ",
            "B = 3 LSF bit(s) of ComboOperand: ",
            "Jump to operand: ",
            "B ^= C ",
            "Output 3 LSF bits of GetComboOperand: ",
            "Ignore ComboOperand LSF bit(s) of B: ",
            "Ignore ComboOperand LSF bit(s) of C: ",
        };

        static void Main(string[] args)
        {
            List<long> initialRegisters;
            List<int> program;
            using (var reader = new StreamReader("Input.txt"))
            {
                var input = reader.ReadToEnd().ReplaceLineEndings().Split("\r\n\r\n").ToList();
                initialRegisters = input[0].Split("\r\n").Select(x => long.Parse(x.Split(": ")[1])).ToList();
                program = input[1].Split(": ")[1].Split(',').Select(int.Parse).ToList();
            }
            var watch = new Stopwatch();

            watch.Start();
            var output = Part1(initialRegisters[0], program);
            watch.Stop();
            Console.WriteLine($"Part 1: {output}, Time: {watch.Elapsed.TotalMicroseconds} μs");

            watch.Restart();
            var output1 = Part2(initialRegisters, program);
            watch.Stop();
            Console.WriteLine($"Part 2: {output1}, Time: {watch.Elapsed.TotalMicroseconds} μs");
        }

        static string Part1(long a, List<int> program)
        {
            A = a; B = 0; C = 0;
            InstructionPntr = 0;
            var max = program.Count;
            int currentInstrPntr;

            while (InstructionPntr < max)
            {
                currentInstrPntr = InstructionPntr;
                Operations[program[currentInstrPntr]](program[currentInstrPntr + 1]);
                if (ShowProgram) //use if you want to see which instructions the input program loops through
                {
                    Console.WriteLine(InstructionExplanations[program[currentInstrPntr]] + program[currentInstrPntr + 1] + ((program[currentInstrPntr] == 3) ? " " + (InstructionPntr != currentInstrPntr) : ""));
                    if (program[currentInstrPntr] == 5)
                        Console.WriteLine("A: " + Convert.ToString(A, 8) + "  " + String.Join(',', Output));
                }


                if (currentInstrPntr == InstructionPntr)
                    InstructionPntr += 2;
            }

            return String.Join(',', Output);
        }
        
        static long Part2 (List<long> initialRegisters, List<int> program)
        {
           Output.Clear();
           return FindA("", program);
        }

        static long FindA(string a, List<int> program)
        {
            if (Output.SequenceEqual(program))
                return Convert.ToInt64(a, 8);

            for (int i = 0; i < 8; i++)
            {
                var nAns = Convert.ToInt64(a + i, 8);
                Output.Clear();
                Part1(nAns, program);
                if (Output.SequenceEqual(program.TakeLast(a.Length + 1)))
                {
                    var output = FindA(a + i, program);
                    if (output != -1)
                        return output;
                }
            }

            return -1;
        }


    }
}
