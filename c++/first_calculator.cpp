#include <iostream>

int main() {

// define arguments
double Arg1, Arg2;
char Operator;
double Result;

std::cout<<"Please enter an expression in the form of argument operator argument\n";
std::cout<<"Example: 8 * 2\n";
std::cin>>Arg1>>Operator>>Arg2;

//switch case for calculating result
switch (Operator)
{
case '+':
  Result = Arg1 + Arg2;
  std::cout<<Result<<"\n";
  break;

case '/':
if (Arg1 == 0|| Arg2 == 0)
  std::cout<<"ILLEGAL TERM";

Result = Arg1 / Arg2;
std::cout<<Result<<"\n";
break;

case 'x':
case '*':
  Result = Arg1 * Arg2;
  std::cout<<Result<<"\n";
break;

case '-':
  Result = Arg1 - Arg2;
  std::cout<<Result<<"\n";
break;

// in case of invalid operator

default:
  std::cout<<"ILLEGAL EXPRESSION";
  break;
}



}
