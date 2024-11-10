#include <stdio.h>

int main() {
    float csc201, csc205, sta205;
    float total, average, percentage;

    printf("Enter marks for CSC 201: ");
    scanf("%f", &csc201);

    printf("Enter marks for CSC 205: ");
    scanf("%f", &csc205);

    printf("Enter marks for STA 205: ");
    scanf("%f", &sta205);

    total = csc201 + csc205 + sta205;
    average = total / 3;
    percentage = (total / 300) * 100;

    printf("\nTotal Marks: %f", total);
    printf("\nAverage Marks: %f", average);
    printf("\nPercentage: %f%%\n", percentage);

    return 0;
}
