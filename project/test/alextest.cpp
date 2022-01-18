#include <iostream>

void who() {
    std::cout << "friend" << std::endl;
}

void say_hello() {
    std::string greet = "Hello";
    std::cout << greet << " "; 
    who();
}

int main (int argc, char *argv[]) {
    int x = 12;
    say_hello();

    return 0;
}
