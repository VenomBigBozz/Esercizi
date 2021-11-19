#include <iostream>

using namespace std;

//heap memory
int *p_global = new int(2);

//data memory
double data = 0.5;
int global = 10;

void foo()
{
    //stack memory
    int local = 20;

    //heap memory
    int *dynamic = new int(30);
    delete dynamic;
    dynamic = NULL;

    cout << "foo()" << endl;
}

int main()
{
    foo();
}