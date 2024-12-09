//By considering the terms in the Fibonacci sequence whose values do not exceed four million,
//find the sum of the even-valued terms.

#include <stdio.h>
int main()
{
    
int sum = 0;
    int i;
    int f1 = 1;
    int f2 = 2;
    while(f1 < 4000000)
    {
        if(f1 % 2 == 0)
        {
            sum += f1;
        }
        int tmp = f1;
        f1 = f2;
        f2 += tmp;
    }
    printf("%d\n", sum);
    return 0;
}