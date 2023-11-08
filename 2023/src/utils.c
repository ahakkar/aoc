#include "utils.h"
#include <stdlib.h> // for dynamic memory allocation
#include <stdio.h>  // for file I/O

// Reads a file to a string, returns a pointer to the string
char *readFileToString(const char *filepath) {

    FILE *fileStream = fopen(filepath, "r");

    if (fileStream == NULL) {
        fprintf(stderr, "Error: file does not exist\n");
        exit(EXIT_FAILURE);
    }

    // Fseek returns 0 if successful, something else if not
    if(fseek(fileStream, 0, SEEK_END) == 0)
    {
        // Figure out the size of the file, and rewind to beginning
        size_t size = ftell(fileStream);
        rewind(fileStream);

        // Since we know the file of the size, we can allocate memory
        // Plus 1 comes from the null terminator '\0'
        char *str = malloc(size + 1);

        // Read the file, close file stream, return str
        fread(str, 1, size, fileStream);
        str[size] = '\0';
        fclose(fileStream);

        return str;
    }

    // failure
    return NULL;
    
}

/* size_t readFileToArray(const char *filepath, int **array) {
    // Implementation of reading file to array
    return NULL;
}
 */
// Implement other utility functions...