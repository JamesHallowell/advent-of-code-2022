#include <assert.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* read_file_as_string(const char* filename) {
    FILE* file = fopen(filename, "r");
    if (file == NULL) {
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    const long file_size = ftell(file);
    rewind(file);

    char* string = malloc(file_size + 1);
    const size_t bytes_read = fread(string, sizeof(char), file_size, file);
    fclose(file);

    if (bytes_read != file_size) {
        free(string);
        return NULL;
    }

    string[file_size] = '\0';
    return string;
}

typedef struct {
    int start;
    int end;
} range;

typedef struct {
    range first;
    range second;
} pair;

pair parse_pair(const char* string) {
    pair pair;
    sscanf(string, "%d-%d,%d-%d", &pair.first.start, &pair.first.end, &pair.second.start, &pair.second.end);
    return pair;
}

bool range_contains_other(range first, range second) {
    return first.start <= second.start && first.end >= second.end;
}

bool pair_fully_contain_the_other(pair pair) {
    return range_contains_other(pair.first, pair.second) || range_contains_other(pair.second, pair.first);
}

bool pair_overlap_at_all(pair pair) {
    return pair.first.start <= pair.second.end && pair.first.end >= pair.second.start;
}

typedef bool (* should_count_pair)(pair);

int count_pairs_if(const char* input, should_count_pair should_count_pair) {
    char* input_copy = malloc(strlen(input) + 1);
    strcpy(input_copy, input);

    char* line_state = NULL;
    char* line = strtok_r(input_copy, "\n", &line_state);

    int total = 0;
    while (line) {
        const pair pair = parse_pair(line);
        if (should_count_pair(pair)) {
            total++;
        }
        line = strtok_r(NULL, "\n", &line_state);
    }

    free(input_copy);

    return total;
}

int main() {
    char* input = read_file_as_string("input.txt");
    assert(input != NULL);

    printf("%d\n", count_pairs_if(input, pair_fully_contain_the_other));
    printf("%d\n", count_pairs_if(input, pair_overlap_at_all));

    free(input);
    return 0;
}
