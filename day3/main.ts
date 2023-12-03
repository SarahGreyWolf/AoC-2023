export function add(a: number, b: number): number {
    return a + b;
}

// Learn more at https://deno.land/manual/examples/module_metadata#concepts
if (import.meta.main) {
    const filePath = Deno.args[0];
    const text = await Deno.readTextFile(filePath);
    const lines = text.split('\n');
    // Part 1
    console.log("The accumulated value is", findAdjacentNumbers(lines));
}

function findAdjacentNumbers(input: string[]): number {
    let just_found = false;
    let accumulator = 0;
    const test: number[][] = [[]];
    test.push([]);
    input.forEach((line, l_index, l_array) => {
        just_found = false;
        test.push([]);
        Array.from(line).forEach((char, c_index, c_array) => {
            if (isDigit(char)) {
                if (getSymbol(c_index, l_index, c_array, l_array) !== undefined && !just_found) {
                    just_found = true;
                    const result = lookBackAndForth(c_index, c_array);
                    test[l_index + 1].push(result);
                    accumulator += result;
                }
            } else {
                just_found = false;
            }
        });
    });
    console.table(test);
    return accumulator;
}

function lookBackAndForth(col_index: number, col_array: string[]): number {
    const numbers: string[] = [col_array[col_index]];
    for (let i = col_index - 1; i >= 0; i--) {
        if (isDigit(col_array[i]))
            numbers.unshift(col_array[i]);
        else
            break;
    }
    for (let i = col_index + 1; i < col_array.length; i++) {
        if (isDigit(col_array[i]))
            numbers.push(col_array[i]);
        else
            break;
    }
    const result = Number(numbers.join(''));
    return result;
}

function getSymbol(col_index: number, row_index: number, col_array: string[], row_array: string[]): { char: string, row_index: number, col_index: number } | undefined {
    let right = ".";
    let left = ".";
    let top = ".";
    let top_right = ".";
    let top_left = ".";
    let bottom = ".";
    let bottom_right = ".";
    let bottom_left = ".";

    if (col_index < col_array.length - 1)
        right = col_array[col_index + 1];

    if (col_index > 0)
        left = col_array[col_index - 1];

    if (row_index > 0)
        top = row_array[row_index - 1][col_index];

    if (row_index > 0 && col_index < col_array.length - 1)
        top_right = row_array[row_index - 1][col_index + 1];

    if (row_index > 0 && col_index > 0)
        top_left = row_array[row_index - 1][col_index - 1];

    if (row_index < row_array.length - 1)
        bottom = row_array[row_index + 1][col_index];

    if (row_index < row_array.length - 1 && col_index < col_array.length - 1)
        bottom_right = row_array[row_index + 1][col_index + 1];

    if (row_index < row_array.length - 1 && col_index > 0)
        bottom_left = row_array[row_index + 1][col_index - 1];

    if (!isDigit(right) && right != '.')
        return { char: right, row_index: row_index, col_index: col_index + 1 };
    if (!isDigit(left) && left != '.')
        return { char: left, row_index: row_index, col_index: col_index - 1 };
    if (!isDigit(top) && top != '.')
        return { char: top, row_index: row_index - 1, col_index: col_index };
    if (!isDigit(bottom) && bottom != '.')
        return { char: bottom, row_index: row_index + 1, col_index: col_index };
    if (!isDigit(top_right) && top_right != '.')
        return { char: top_right, row_index: row_index - 1, col_index: col_index + 1 };
    if (!isDigit(top_left) && top_left != '.')
        return { char: top_left, row_index: row_index - 1, col_index: col_index - 1 };
    if (!isDigit(bottom_right) && bottom_right != '.')
        return { char: bottom_right, row_index: row_index + 1, col_index: col_index + 1 };
    if (!isDigit(bottom_left) && bottom_left != '.')
        return { char: bottom_left, row_index: row_index + 1, col_index: col_index - 1 };

    return undefined;
}

function isDigit(input: string): boolean {
    return !isNaN(Number(input));
}