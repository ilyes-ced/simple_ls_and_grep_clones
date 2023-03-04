#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int main() {
  
    char input[255];

    printf("personel project \nSQLite3 lite clone\nV 0.0.2");
    while(true){

        printf("\nSQLone > ");
        fgets (input, 255, stdin);

        //removes the line jump from the input
        input[strcspn(input, "\n")] = 0;

        if(strcmp(input[0], ".") == 0){
            switch (do_meta_command(input)) {
                case (META_COMMAND_SUCCESS):
                    continue;
                case (META_COMMAND_UNRECOGNIZED_COMMAND):
                    printf("Unrecognized command '%s'\n", input_buffer->buffer);
                    continue;
            }
        }else{
            printf("-> unrecognized command : '%s'", input);
        }

    }
  
    return 0;
} 