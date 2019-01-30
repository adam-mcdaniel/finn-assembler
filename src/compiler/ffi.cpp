#include <map>
#include <string>
#include <vector>
#include <iostream>
using namespace std;


template<typename Fn>
struct FFI {
    map<string, Fn> functions;

    FFI() {

    }

    FFI add(Fn f, string name) {
        this->functions[name] = f;
        return *this;
    }


    Fn get(string name) {
        return this->functions[name];
    }
};



int square(int n) {
    return n*n;
}


int main() {
    cout << FFI<int (*)(int)>().add(square, "square").get("square")(5) << endl;


    return 0;
}