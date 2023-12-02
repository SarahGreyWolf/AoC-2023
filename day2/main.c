#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef _WIN32
#include <BaseTsd.h>
#define ssize_t SSIZE_T
#else
#include <sys/types.h>
#endif

#pragma clang diagnostic ignored "-Wdeprecated-declarations"

const int max_red = 12;
const int max_green = 13;
const int max_blue = 14;

typedef struct {
    int red;
    int green;
    int blue;
} Round;

int analyse_lines(FILE *fp);
int check_game(char *line, ssize_t len, int part);

int main(int argc, char **argv) {
    if (argc != 2) {
        printf("Usage: %s <filepath>\n", argv[0]);
        return 1;
    }
    FILE *fp = fopen(argv[1], "r");
    if (fp == 0) {
        printf("Failed to open %s\n", argv[1]);
        return 1;
    }

    int result = analyse_lines(fp);
    if (result == -1) {
        return 1;
    }

    // This segfaults for some reason
    // File not found?
    if (fp != 0) {
        fclose(fp);
    }
    return 0;
}

int analyse_lines(FILE *fp) {
    int line_index = 0;
    // This is bad, bad coding practice, very bad, but for this I don't care
    char **lines = malloc(10000);
    size_t len;
    ssize_t *read = malloc(10000);
    while ((read[line_index] = getline(&lines[line_index], &len, fp)) != -1) {
        printf("Read %lu for line %d\n", read[line_index], line_index);
        line_index++;
    }

    // PART 1
    int accumulated = 0;

    for (int i = 0; i < line_index; i++) {
        int id = check_game(lines[i], read[i], 1);
        if (id == -1) {
            free(lines);
            return -1;
        }
        printf("ID: %d\n", id);
        accumulated = accumulated + id;
    }

    printf("The accumulated value is %d\n\n", accumulated);

    accumulated = 0;
    for (int i = 0; i < line_index; i++) {
        int result = check_game(lines[i], read[i], 2);
        if (result == -1) {
            free(lines);
            return -1;
        }
        printf("Squared result for %d: %d\n", i + 1, result);
        accumulated = accumulated + result;
    }

    printf("The accumulated square is %d\n", accumulated);

    return line_index;
}

int check_game(char *line, ssize_t len, int part) {
    // All rounds for a line, expecting 10 or less
    Round *round = malloc(sizeof(Round) * 10);

    int game_id = -1;
    int rounds = 0;

    char *id_characters = malloc(3 * sizeof(char));
    int index = 0;
    size_t round_starts = -1;

    // Get game id
    for (int c = 0; c < len; c++) {
        if (isdigit(line[c])) {
            id_characters[index++] = line[c];
            if (index > 3) {
                char *new_ptr = realloc(id_characters, index * sizeof(char));
                if (new_ptr == NULL) {
                    printf("Failed to realloc id_characters\n");
                    free(round);
                    free(id_characters);
                    return 1;
                }
                id_characters = new_ptr;
            }
        }
        if (line[c] == ':') {
            round_starts = c + 2;
            break;
        }
    }

    if (round_starts == -1) {
        printf("Could not find where the rounds info starts\n");
        return -1;
    }

    game_id = atoi(id_characters);
    free(id_characters);

    if (game_id == -1) {
        printf("Game ID could not be found\n");
        return -1;
    }
    printf("Game ID: %d\n", game_id);

    char *colour_char = malloc(3 * sizeof(char));
    if (colour_char == NULL) {
        printf("Failed to malloc colour_char string\n");
        return -1;
    }

    for (int i = 0; i < 3; i++) {
        colour_char[i] = ' ';
    }
    // Just assuming this elf doesn't have a magical bag that can hold more than
    // 999 dice, bad practice
    char **numbers = malloc(3 * sizeof(char) * 3);
    if (numbers == NULL) {
        printf("Failed to malloc numbers string\n");
        return -1;
    }

    for (int i = 0; i < 3; i++) {
        numbers[i] = malloc(3 * sizeof(char));
    }

    index = 0;
    int colour_index = 0;
    for (int c = round_starts - 1; c < len; c++) {
        if (isdigit(line[c]) && colour_char[colour_index] == ' ') {
            numbers[colour_index][index] = line[c];
            index++;
        }
        if (!isdigit(line[c]) && line[c] != ' ' &&
            colour_char[colour_index] == ' ') {
            index = 0;
            colour_char[colour_index] = line[c];
            colour_index++;
            if (line[c] == 'r')
                c = c + 3;
            if (line[c] == 'g')
                c = c + 5;
            if (line[c] == 'b')
                c = c + 4;
        }
        if (line[c] == ';' || line[c] == '\n') {
            int red = 0;
            int green = 0;
            int blue = 0;
            for (int i = 0; i < 3; i++) {
                if (colour_char[i] == 'r') {
                    red = atoi(numbers[i]);
                }
                if (colour_char[i] == 'g') {
                    green = atoi(numbers[i]);
                }
                if (colour_char[i] == 'b') {
                    blue = atoi(numbers[i]);
                }
            }
            round[rounds].red = red;
            round[rounds].green = green;
            round[rounds].blue = blue;
            for (int i = 0; i < 3; i++) {
                colour_char[i] = ' ';
            }
            for (int i = 0; i < 3; i++) {
                free(numbers[i]);
                numbers[i] = malloc(3 * sizeof(char));
            }
            rounds++;
            index = 0;
            colour_index = 0;
        }
    }

    if (part == 1) {
        int failed = 0;
        for (int i = 0; i < rounds; i++) {
            if (round[i].red > max_red) {
                failed = 1;
                break;
            }
            if (round[i].green > max_green) {
                failed = 1;
                break;
            }
            if (round[i].blue > max_blue) {
                failed = 1;
                break;
            }
        }
        if (!failed) {
            free(round);
            return game_id;
        }
    }
    if (part == 2) {
        int most_red = 0;
        int most_green = 0;
        int most_blue = 0;
        for (int i = 0; i < rounds; i++) {
            if (round[i].red > most_red)
                most_red = round[i].red;
            if (round[i].green > most_green)
                most_green = round[i].green;
            if (round[i].blue > most_blue)
                most_blue = round[i].blue;
        }
        free(round);
        return most_red * most_green * most_blue;
    }

    free(round);
    return 0;
}