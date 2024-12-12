#ifndef UTILS_H
#define UTILS_H

#include <stddef.h> // for size_t

// Function prototype for reading a file to a string
char *readFileToString(const char *filepath);

// Function prototype for reading a file to an array
size_t readFileToArray(const char *filepath, int **array);

// Other utility function prototypes...

#endif // UTILS_H