#include <iostream>

void say_hello() {
    std::string greet = "Hello";
    std::string who = "Friend";
    std::cout << greet << " " << who << std::endl;
}

int main (int argc, char *argv[]) {
    int x = 12;
    say_hello();

    return 0;
}
