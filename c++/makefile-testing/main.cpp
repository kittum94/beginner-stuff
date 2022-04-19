#include "header.h"

int main() {

int input;

std::cout<<"Please enter a number.\n";
std::cin>>input;

int inputfact = factorial(input);

std::cout<<"The factorial of "<<input<<" is "<<inputfact<<"\n";

}