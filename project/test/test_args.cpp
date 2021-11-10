#include <iostream>

int main(int argc, char *argv[]) {

    std::cout << "Hello friend " << argc << std::endl;
    for (int i=0; i<argc; i++)
        std::cout << "Arg " << i << " is " << argv[i] << ";" <<std::endl;

    return 0;
}
