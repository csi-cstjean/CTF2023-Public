#include <sys/sendfile.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <fcntl.h>

//load flag from ENV
#define FLAG_SIZE 35

char data[12] = "aAYX$3202-d3";

void showFlag()
{
    int fd = open("/flag", O_RDONLY);
    if (fd == -1) {
        perror("open");
        exit(EXIT_FAILURE);
    }
    sendfile(STDOUT_FILENO, fd, 0, FLAG_SIZE);
    close(fd);
    exit(EXIT_SUCCESS);
}


void secondStep() {
    printf("Bravo, tu es un des nôtres !\n");
    showFlag();
}


struct PasswordFuzzing{
    char firt_part[10];
    char second_part[10];
    int  holder;
    char end_part[10];
} password_fuzzing;


int checkFirstPart(char *first_part)
{
    char something[10] = "5S0c!3t3S3";

    for (int i = 0; i < 10; i++)
    {
        if (first_part[i] != something[i])
        {
            return 0;
        }
    }

    return 1;
}


int gdbBlocker() {
    int a = 0;
    int b = 0;
    int c = 0;
    int d = 0;
    int e = 0;

    if (a == 0)
    {
        a=1;
    }
    if (b == 0)
    {
        b=1;
    }
    if (c == 0)
    {
        c=1;
    }
    if (d == 0)
    {
        d=1;
    }
    if (e == 0)
    {
        e=1;
    }
    return 1;
}

int checkSecondPart(int *data) {
    for (int i = 0; i < 3; i++)
    {

        //show in binary
        for (int j = 0; j < 32; j++)
        {
            gdbBlocker();
        }
    }

    if (data[0] != 1949528675)
    {
        return 0;
    }
    if (data[1] != 1079980884)
    {
        return 0;
    }
    if (data[2] != 594309229)
    {
        return 0;
    }
    return 1;

}


void checkPassword()
{
    char password[36];
    printf("Enter password: ");
    scanf("%s", password);

    //convert the password to a struct

    memcpy(&password_fuzzing, password, sizeof(password_fuzzing));
    int *centinel = malloc(sizeof(int));
    *centinel = 560341868;
    if (checkSecondPart((int*)password_fuzzing.second_part) == 0)
    {
        printf("Désolé, mauvais mot de passe !\n");
        return;
    }

    if (checkFirstPart(password_fuzzing.firt_part) == 0)
    {
        printf("Désolé, mauvais mot de passe !\n");
        return;
    }


    if (password_fuzzing.holder == *centinel)
    {
        // invert data
        for (int i = 11; i >= 2; i--)
        {
            if (data[i] != password_fuzzing.end_part[11-i])
            {
                printf("Désolé, mauvais mot de passe !\n");
                return;
            }
        }

        secondStep();
    }
    else
    {
        printf("Désolé, mauvais mot de passe !\n");
    }
}



void init() {
    setvbuf(stdin, NULL, _IONBF, 0);
    setvbuf(stdout, NULL, _IONBF, 0);
    setvbuf(stderr, NULL, _IONBF, 0);
}

int main(int argc, char *argv[])
{
    init();
    printf("                    T~~\n");
    printf("                    |\n");
    printf("                    |\n");
    printf("                    /\"\\\n");
    printf("              T~~  |'|   T~~\n");
    printf("              |    T~ W  |\n");
    printf("              |    |  |  |\n");
    printf("    T~ T~~    |    |  |  |    T~~  T~\n");
    printf("    |  |      |    |  |  |    |    |\n");
    printf("    |  |      |    |  |  |    |    |\n");
    printf("    W  |      |    |  W  |    |    |\n");
    printf("_______|______W______|___W____W____|________\n");
    printf("|    __        ___           ___        ___|\n");
    printf("|__________________________________________|\n");
    printf("|                                          |\n");
    printf("|      Bravo, voici votre dernier test     |\n");
    printf("|__________________________________________|\n");
    printf("|            Code du systeme :             |\n");
    printf("|__________________________________________|\n");
    checkPassword();
    return 0;
}
