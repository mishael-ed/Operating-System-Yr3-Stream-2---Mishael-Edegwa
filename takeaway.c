#include <stdio.h>
#include <ctype.h>

int main() {
    char choice;
    printf("Enter the type of input (c for character, i for integer, f for float): ");
    scanf(" %c", &choice);

    switch (choice) {
        case 'c': {
            char input_char;
            printf("Enter a character: ");
            scanf(" %c", &input_char);
            
            printf("The next 4 characters are: ");
            for (int i = 1; i <= 4; i++) {
                printf("%c ", input_char + i);
            }
            printf("\n");

            printf("ASCII code: %d\n", input_char);
            printf("Size of character: %zu bytes\n", sizeof(input_char));
            break;
        }
        
        case 'i': {
            int input_int;
            printf("Enter an integer: ");
            scanf("%d", &input_int);

            printf("The next 4 integers (in multiples of 3) are: ");
            for (int i = 1; i <= 4; i++) {
                printf("%d ", input_int + (i * 3));
            }
            printf("\n");

            printf("Size of integer: %zu bytes\n", sizeof(input_int));
            break;
        }
        
        case 'f': {
            float input_float;
            printf("Enter a float: ");
            scanf("%f", &input_float);

            printf("The next 4 floats in multiples of 3 are: ");
            for (int i = 1; i <= 4; i++) {
                printf("%.2f ", input_float + (i * 3.0));
            }
            printf("\n");

            printf("Size of float: %zu bytes\n", sizeof(input_float));
            break;
        }

        default:
            printf("Invalid input type! Please enter 'c', 'i', or 'f'.\n");
    }

    return 0;
}
