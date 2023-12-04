using System.Collections;

class Day4 {

    static void Main(string[] args) {
        if (args.Length != 1) {
            Console.WriteLine("Usage: day4 <filepath>");
        }
        string[] lines = File.ReadAllLines(args[0]);

        int card_sum = Part1(lines);
        Console.WriteLine("The sum of all the cards points is " + card_sum);


    }

    public static int Part1(string[] lines) {
        List<int> cards_worth = new List<int>();
        foreach (string line in lines) {
            string[] split_from_game = line.Split(':');
            string game_name = split_from_game[0];
            string game_string = new string(game_name.SkipWhile((character) => !char.IsDigit(character)).ToArray());
            int game = int.Parse(game_string);
            string[] split_numbers = split_from_game[1].Split('|');
            List<int> player_numbers = GetNumbers(split_numbers[0]);
            if (player_numbers.Count == 0) {
                Console.WriteLine("No player numbers could be found?");
            }
            List<int> winning_numbers = GetNumbers(split_numbers[1]);
            if (winning_numbers.Count == 0) {
                Console.WriteLine("No winning numbers could be found?");
            }
            var result = player_numbers.Aggregate(0, (acc, value) => winning_numbers.Contains(value) ? acc + 1 : acc);
            if (result > 2) {
                cards_worth.Add((int)Math.Pow(2, result - 1));
            } else {
                cards_worth.Add(result);
            }
        }
        return cards_worth.Aggregate(0, (acc, value) => acc + value);
    }

    public static List<int> GetNumbers(string numbers) {
        List<int> result = new List<int>();

        List<string> split_whitespace = numbers.Split(' ').ToList();
        split_whitespace.RemoveAll(String.IsNullOrEmpty);
        foreach (string number in split_whitespace) {
            string found = new string(number.TakeWhile(char.IsDigit).ToArray());
            result.Add(int.Parse(found));
        }

        return result;
    }

}