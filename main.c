#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int main() {
  
    char input[255];

    printf("personel project \nSQLite3 lite clone\nV 0.0.2");
    while(true){
        printf("\nSQLone > ");
        scanf("%s", input);

        if(strcmp(input, ".exit") == 0){
            exit(EXIT_SUCCESS);
        }else{
            printf("%s", input);
        }
    }
  
    return 0;
} 