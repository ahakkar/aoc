#include "main.h"
#include "utils.h"
#include "01/task1.h"

#include <stdio.h>
#include <stdlib.h>



int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <integer>\n", argv[0]);
        return EXIT_FAILURE;
    }

    int taskNumber = atoi(argv[1]);
    if (taskNumber < 1 || taskNumber > 24) {
        fprintf(stderr, "Error: task_number must be between 1 and 24.\n");
        return EXIT_FAILURE;
    }

    switch (taskNumber) {
        case 1:
            task1();
            break;
        default:
            fprintf(stderr, "Error: Unknown task number %d.\n", taskNumber);
            return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}