#include <stdio.h>


extern int add(int, int);

int main(int argc, char const *argv[])
{
    int x = 1, y = 2;
    int z = add(x, y);
    printf("%d + %d = %d\n", x, y, z);
    return 0;
}

