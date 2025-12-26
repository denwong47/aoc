#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "types.c"

USIZE read_line(INPUT_LINE buffer) {
    size_t len=MAX_LINE_LENGTH;
    size_t read=getline(&buffer, &len, stdin);
    if (read == -1) {
        return 0;
    }
    buffer[read-1] = '\0';
    return (USIZE) read;
}

#ifdef UNIT_TEST
int main() {
    INPUT_LINE buffer = (INPUT_LINE)malloc(MAX_LINE_LENGTH*sizeof(char));
    USIZE read;
    for (size_t i=0; i<=5; i++) {
        read = read_line(buffer);
        if (read == 0) {
            printf("Nothing to read.\n");
        } else {
            printf("Read %u bytes from line: %s\n", read, buffer);
        }
    }
}
#endif
