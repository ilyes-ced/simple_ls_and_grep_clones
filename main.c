#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>




int command(char[] statement){
    switch (statement) {
        case (".exit"):
            printf("exiting now");
            exit(EXIT_SUCCESS);
            break;
        case (".hello"):
            printf("Unrecognized command '%s'\n", statement);
            continue;
    }
    return 0;
}



int main() {
  
    char input[255];

    printf("personel project \nSQLite3 lite clone\nV 0.0.2");
    while(true){

        printf("\nSQLone > ");
        fgets (input, 255, stdin);

        //removes the line jump from the input
        input[strcspn(input, "\n")] = 0;

        if(strcmp(input[0], ".") == 0){
            command(input);
        }else{
            printf("-> unrecognized command : '%s'", input);
        }

    }
  
    return 0;
} 