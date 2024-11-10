#include <stdio.h>
#include <ctype.h>

int main() {
    char input_char;

    printf("Enter a character between A and J: ");
    scanf(" %c", &input_char);
    input_char = toupper(input_char);

    if (input_char >= 'A' && input_char <= 'J') {
        printf("The next 6 characters are: ");
        
        for (int i = 1; i <= 6; i++) {
            printf("%c ", input_char + i);
        }
        
        printf("\n");
    } else {
        printf("Invalid input! Please enter a character between A and J.\n");
    }

    return 0;
}
