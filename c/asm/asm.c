//https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html

#include <stdio.h>

int main() {
	int src = 1;
	int dst;   
	
	asm ("mov %1, %0;"   // dst  = 1
	     "inc %0;"       // dst += 1
	     "add $2, %0;"   // dst += 2 
	     : "=r" (dst)    // OutputOperands
	     : "r"  (src));  // InputOperands

	printf("%d\n", dst); // 4

	return 0;
}

