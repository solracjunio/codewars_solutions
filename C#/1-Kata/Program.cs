using System;

namespace StringSplit
{
    class Program
    {
        static void Main(string[] args)
        {
            string inputString = "This is a sample string";
            string[] words = SplitStringIntoWords(inputString);

            Console.WriteLine("Input String: " + inputString);
            Console.WriteLine("Words: ");
            foreach (string word in words)
            {
                Console.WriteLine(word);
            }
        }

        static string[] SplitStringIntoWords(string str)
        {
            return str.Split(' ');
        }
    }
}
