using System.Collections;
using System.Threading;

class Day4 {

    static void Main(string[] args) {
        if (args.Length != 1) {
            Console.WriteLine("Usage: day4 <filepath>");
        }
        string[] lines = File.ReadAllLines(args[0]);

        int card_sum = Part1(lines);
        Console.WriteLine("The sum of all the cards points is " + card_sum);

        int total_cards = Part2(lines);
        Console.WriteLine("The total number of cards is " + total_cards);

    }

    public static int Part2(string[] lines) {
        List<Card> initial_cards = new List<Card>();
        int[] played = new int[lines.Length];
        foreach (string line in lines) {
            Card card = new Card(line);
            initial_cards.Add(card);
        }
        int index = 0;
        foreach (Card card in initial_cards) {
            int winning = card.GetWinnings();
            played[index] += 1;
            for (int k = 0; k < winning; k++) {
                played[index + k + 1] += played[index];
            }
            index += 1;
        }

        return played.Sum();
    }

    public static int Part1(string[] lines) {
        List<int> cards_worth = new List<int>();
        foreach (string line in lines) {
            cards_worth.Add(GetCardResults(line));
        }
        return cards_worth.Aggregate(0, (acc, value) => acc + value);
    }

    public static int GetCardResults(string line) {
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
            return (int)Math.Pow(2, result - 1);
        } else {
            return result;
        }
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

class Card {
    public int Id;
    public List<int> PlayerNumbers = new List<int>();
    public List<int> WinningNumber = new List<int>();

    public Card(string line) {
        string[] split_from_game = line.Split(':');
        string game_name = split_from_game[0];
        string game_string = new string(game_name.SkipWhile((character) => !char.IsDigit(character)).ToArray());
        int game = int.Parse(game_string);
        Id = game;
        string[] split_numbers = split_from_game[1].Split('|');
        PlayerNumbers = Day4.GetNumbers(split_numbers[0]);
        if (PlayerNumbers.Count == 0) {
            Console.WriteLine("No player numbers could be found?");
        }
        WinningNumber = Day4.GetNumbers(split_numbers[1]);
        if (WinningNumber.Count == 0) {
            Console.WriteLine("No winning numbers could be found?");
        }
    }

    public int GetWinnings() {
        return PlayerNumbers.Aggregate(0, (acc, value) => WinningNumber.Contains(value) ? acc + 1 : acc);
    }
}