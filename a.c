int a = 10;
#include "stdio.h"
typedef void f(int);

f *fn(int a) {
    printf("fuck me\n");
    return (f *)&fn;
}

int main() {
    f *ff = fn(1);
    ff(1);

    printf("%d", a);
}