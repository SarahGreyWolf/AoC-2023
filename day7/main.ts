const SCORE: { [key: string]: number } = {
    '2': 2,
    '3': 3,
    '4': 4,
    '5': 5,
    '6': 6,
    '7': 7,
    '8': 8,
    '9': 9,
    'T': 10,
    'J': 11,
    'Q': 12,
    'K': 13,
    'A': 14
};

enum Hands {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

async function main() {
    if (Deno.args.length == 0 || Deno.args.length > 1) {
        console.log("Usage: deno run main.ts <filepath>");
    }
    const filePath = Deno.args[0];
    const text = await Deno.readTextFile(filePath);
    const lines = text.split("\n");

    const scored_lines: [{ scored: number[], bid: number }] = [{ scored: [0], bid: 0 }];
    scored_lines.pop();

    lines.forEach(line => {
        const split = line.split(' ');
        const bid = Number(split[1]);
        const scored: number[] = [];
        split[0].split('').forEach(char => {
            scored.push(SCORE[char]);
        });
        scored_lines.push({ scored, bid });
    });
    scored_lines.sort((a, b): number => {
        const a_classification = classify(a.scored);
        const b_classification = classify(b.scored);
        if (a_classification === b_classification) {
            for (let i = 0; i < 5; i++) {
                if (a.scored[i] == b.scored[i])
                    continue;
                return a.scored[i] - b.scored[i];
            }
        }
        return a_classification - b_classification;
    });
    scored_lines.forEach(element => {
        const classification = classify(element.scored);
        console.log(element, ":", classification)
    });
    let accumulator = 0;
    scored_lines.forEach((element, idx) => {
        accumulator += element.bid * (idx + 1);
    });
    console.log("The total winnings is ", accumulator);
}

if (import.meta.main) {
    main();
}

function GreaterThan(a: { scored: number[], bid: number }, b: { scored: number[], bid: number }): boolean {
    const a_classification = classify(a.scored);
    const b_classification = classify(b.scored);
    if (a_classification > b_classification) {
        return true;
    } else {
        return false;
    }
}

function LessThan(a: { scored: number[], bid: number }, b: { scored: number[], bid: number }): boolean {
    return !GreaterThan(a, b) && !EqualTo(a, b);
}

function EqualTo(a: { scored: number[], bid: number }, b: { scored: number[], bid: number }): boolean {
    const a_classification = classify(a.scored);
    const b_classification = classify(b.scored);
    if (a_classification === b_classification) {
        return true;
    } else {
        return false;
    }
}

function classify(scored: number[]): Hands {
    const values: number[] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    scored.forEach(value => {
        values[value - 1] += 1;
    });

    let result = Hands.HighCard;

    if (scored[0] === 2 && scored[1] === 2 && scored[2] == 8 && scored[3] == 2 && scored[4] == 8) {
        console.log(values);
    }

    let index = 0;
    for (const element of values) {
        if (element == 5) {
            result = Hands.FiveOfAKind;
            break;
        }
        if (element == 4) {
            result = Hands.FourOfAKind;
            break;
        }
        if (element == 3) {
            if (values.find((value, _idx, _obj) => value == 2) !== undefined) {
                result = Hands.FullHouse;
                console.log(result);
                break;
            } else {
                result = Hands.ThreeOfAKind;
                console.log(result);
            }
        }
        if (element == 2) {
            if (values.find((value, idx, _obj) => index != idx && value == 2) !== undefined) {
                result = Hands.TwoPair;
                console.log(result);
                break;
            } else {
                result = Hands.OnePair;
            }
        }
        index += 1;
    }
    return result;
}