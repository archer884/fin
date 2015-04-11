using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Fin
{
    class Program
    {
        static void Main(string[] args)
        {
            if (args.Length != 2)
            {
                Console.WriteLine("USAGE:\n\t./Fin.exe <wordlist> <prefix>");
                return;
            }

            var words = LoadWords(args[0]);
            var desc = GetDescendants(args[1], words);

            foreach (var d in desc)
            {
                Console.WriteLine(d);
            }
        }

        static IEnumerable<string>GetDescendants(string target, List<string> words)
        {
            int index = words.BinarySearch(target);
            index = index > 0 ? index : ~index;

            for (int i = index; ; i++)
            {
                if (words[i].StartsWith(target))
                {
                    yield return words[i];
                }
                else break;
            }
        }

        static List<string> LoadWords(string path)
        {
            var list = File.ReadLines(path).ToList();
            list.Sort();
            return list;
        }
    }
}
