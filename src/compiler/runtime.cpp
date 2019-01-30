#include <map>
#include <string>
#include <vector>
#include <iostream>
using namespace std;

template<typename T>
T concat_vector(T a, T b) {
    a.insert(a.end(), b.begin(), b.end());
    return a;
}

template<typename K, typename V>
void print_map(map<K,V> m)
{
    for (auto pair: m) {
        cout << " (" << pair.first << ": " ;
        pair.second.print();
        cout << ") ";
    }
}


void assertion(bool assertion, string message) {
    if (not assertion) {
        cout << "Error: " << message << endl;
        exit(1);
    }
}


enum Type {
    String, Number, List, Instance, None, Instruction
};

enum InstructionCode {
    Print, Println, Add, Mul, Sub, Div, Mod, Call, Load, Store, GetAttr, SetAttr, Pass
};


template<typename A>
struct Table {
    map<string, A> contents;

    void set(string name, A object) {
        contents[name] = object;
    }

    A get(string name) {
        return this->contents[name];
    }

    bool is_in(string name) {
        return this->contents.find(name) != this->contents.end();
    }

    void print() {
        print_map(contents);
    }
};


typedef vector<double> RawObject;
struct Object {
    Type type;
    RawObject contents;
    vector<Object> list;
    Table<Object> table;

    Object() {
        this->type = None;
    }

    Object(InstructionCode i) {
        switch (i) {
            case Print: this->set_contents(0); break;
            case Println: this->set_contents(1); break;
            case Add: this->set_contents(2); break;
            case Mul: this->set_contents(3); break;
            case Sub: this->set_contents(4); break;
            case Div: this->set_contents(5); break;
            case Mod: this->set_contents(6); break;
            case Call: this->set_contents(7); break;
            case Load: this->set_contents(8); break;
            case Store: this->set_contents(9); break;
            case GetAttr: this->set_contents(10); break;
            case SetAttr: this->set_contents(11); break;
            case Pass: this->set_contents(12); break;
            default: assertion(false, "Invalid instruction"); break;
        }
        this->type = Instruction;
    }

    Object(string s) {
        this->type = String;
        this->contents = RawObject(s.begin(), s.end());
    }

    Object(double n) {
        this->type = Number;
        this->contents.push_back(n);
    }

    Object(vector<Object> contents) {
        this->type = List;
        this->list = contents;
    }

    Object(Type type, RawObject contents) {
        this->type = type;
        this->contents = contents;
    }

    void set_type(Type type) {
        this->type = type;
    }

    template<typename T>
    void set_contents(T t) {
        Object d = Object(t);
        this->type = d.get_type();
        this->contents = d.get_contents();
    }

    Object get_attr(string name) {
        // cout << "Getting attr " << name << endl;
        return this->table.get(name);
    }

    void set_attr(Object object, string name) {
        this->set_type(Instance);
        this->table.set(name, object);
    }

    Type get_type() {
        return this->type;
    }

    RawObject get_contents() {
        return this->contents;
    }

    double as_number() {
        return this->contents[0];
    }

    string as_string() {
        return string(this->contents.begin(),this->contents.end());
    }

    vector<Object> as_list() {
        return this->list;
    }

    void list_push(Object data) {
        this->set_type(List);
        this->list.push_back(data);
    }

    Object list_pop() {
        Object back = this->list.back();
        this->list.pop_back();
        return back;
    }

    void print() {
        switch (this->get_type()) {
            case String:
                cout << this->as_string();
                break;
            case Number:
                cout << this->as_number();
                break;
            case List:
                cout << "[";

                // print each object in list
                for (int i=0; i < this->as_list().size(); i++) {
                    this->as_list()[i].print();
                    if (i < this->as_list().size()-1) {
                        cout << ", ";
                    }
                }

                cout << "]";
                break;
            case Instance:
                // cout << "Object";
                cout << "Object[";
                this->table.print();
                cout << "]";
                break;
            case None:
                cout << "None";
                break;
                
            default: break;
        }
    }

    void println() {
        this->print();
        cout << endl;
    }

    friend Object operator+(Object a, Object b) {
        assertion(
            a.get_type() == b.get_type(), "Tried to add objects of two different types"
        );
        assertion(
            a.get_type() == String || a.get_type() == Number || a.get_type() == List,
            "Tried to add objects not of type String, Number, or List"
        );

        switch (a.get_type()) {
            case String:
                return Object(a.as_string() + b.as_string());
                break;
            case Number:
                return Object(a.as_number() + b.as_number());
                break;
            case List:
                return Object(
                    concat_vector(a.as_list(), b.as_list())
                    );
                break;
            default:
                return Object();
                break;
        }
    }

    friend Object operator-(Object a, Object b) {
        assertion(
            a.get_type() == b.get_type(), "Tried to subtract objects of two different types"
        );
        assertion(
            a.get_type() == Number,
            "Tried to subtract non-Number"
        );

        return Object(a.as_number() - b.as_number());
    }

    friend Object operator*(Object a, Object b) {
        assertion(
            a.get_type() == b.get_type(), "Tried to multiply objects of two different types"
        );

        assertion(
            a.get_type() == Number,
            "Tried to multiply non-Number"
        );

        return Object(a.as_number() * b.as_number());
    }

    friend Object operator/(Object a, Object b) {
        assertion(
            a.get_type() == b.get_type(), "Tried to divide objects of two different types"
        );

        assertion(
            a.get_type() == Number,
            "Tried to divide non-Number"
        );

        return Object(a.as_number() / b.as_number());
    }

    friend Object operator%(Object a, Object b) {
        assertion(
            a.get_type() == b.get_type(), "Tried to divide objects of two different types"
        );

        assertion(
            a.get_type() == Number,
            "Tried to divide non-Number"
        );

        return Object(int(a.as_number()) % int(b.as_number()));
    }

    friend Object operator==(Object a, Object b) {
        return Object(int(a.get_type() == b.get_type() && a.get_contents() == b.get_contents()));
    }

    friend Object operator!=(Object a, Object b) {
        return Object(int(!bool((a == b).as_number())));
    }
};


struct Scope {
    Table<pair<Object, Scope>> table;
    Scope *outer_scope;

    Scope() {}

    Scope(Scope* scope) {
        outer_scope = scope;
    }

    bool is_bottom_scope() {
        return outer_scope == NULL;
    }

    void define(string name, pair<Object, Scope> object) {
        this->table.set(name, object);
    }

    pair<Object, Scope> get(string name) {
        // if name is in current scope
        if (this->table.is_in(name)) {
            return this->table.get(name);
        } 
        // else, check inner scope
        else {
            assertion(!this->is_bottom_scope(), name + " not defined");
            return this->outer_scope->get(name);
        }
    }
};


struct StackFrame {
    Scope scope = Scope();
    vector<pair<Object, Scope>> contents;
    Object instructions;
    StackFrame* outer_stack;

    StackFrame() {}
    
    StackFrame(Scope scope) {
        this->scope = Scope(scope);
    }
    
    StackFrame(Object instructions) {
        this->instructions = instructions;
    }

    StackFrame(Scope scope, Object instructions) {
        this->scope = Scope(scope);
        this->instructions = instructions;
    }

    StackFrame(StackFrame *stack, Scope scope, Object instructions) {
        this->outer_stack = stack;
        this->scope = Scope(scope);
        this->instructions = instructions;
    }
    
    void run() {
        pair<Object, Scope> object_and_scope;
        Object object;
        Object data;
        string name;
        for (Object instruction : this->instructions.as_list()) {
            switch (instruction.get_type()) {
                case Instruction:
                    switch (int(instruction.as_number())) {
                        case 0: this->pop_value().print(); break;
                        case 1: this->pop_value().println(); break;
                        case 2: this->push(this->pop_value() + this->pop_value()); break;
                        case 3: this->push(this->pop_value() * this->pop_value()); break;
                        case 4: this->push(this->pop_value() - this->pop_value()); break;
                        case 5: this->push(this->pop_value() / this->pop_value()); break;
                        case 6: this->push(this->pop_value() % this->pop_value()); break;
                        case 7: this->call(this->pop()); break;
                        case 8:
                            // object_and_scope = this->load(this->pop().as_string());
                            this->push(this->load(this->pop_value().as_string())); break;
                        case 9: this->store(this->pop(), this->pop_value().as_string()); break;
                        case 10:
                            name = this->pop_value().as_string();
                            object = this->pop_value();
                            this->push(object.get_attr(name));
                            break;
                        case 11:
                            name = this->pop_value().as_string();
                            data = this->pop_value();
                            object = this->pop_value();
                            object.set_attr(data, name);
                            this->push(object);
                            break;
                        case 12: break;
                    }
                    break;

                default:
                    // cout << "data: ";
                    // instruction.println();
                    // cout << "type: " << instruction.get_type() << endl;
                    this->push(instruction);
                    break;
            }
        }
    }

    bool is_empty() {
        return this->contents.empty();
    }

    bool has_outer_stack() {
        return this->outer_stack != NULL;
    }

    void call(pair<Object, Scope> object_and_scope) {
        StackFrame s = StackFrame(this, object_and_scope.second, object_and_scope.first);
        s.run();
        while (!s.is_empty()) {
            this->push(s.pop());
        }
    }

    pair<Object, Scope> load(string name) {
        return this->scope.get(name);
    }

    void store(pair<Object, Scope> object, string name) {
        this->scope.define(name, object);
    }

    void push(Object object) {
        this->contents.push_back(make_pair(object, this->scope));
    }

    void push(Object object, Scope scope) {
        this->contents.push_back(make_pair(object, scope));
    }

    void push(pair<Object, Scope> object_and_scope) {
        this->contents.push_back(object_and_scope);
    }

    pair<Object, Scope> pop() {
        if (this->is_empty() && this->has_outer_stack()) {
            return this->outer_stack->pop();

        } else {
            pair<Object, Scope> back = this->contents.back();
            this->contents.pop_back();
            return back;
        }
    }

    Object pop_value() {
        if (this->is_empty() && this->has_outer_stack()) {
            return this->outer_stack->pop().first;

        } else {
            pair<Object, Scope> back = this->contents.back();
            this->contents.pop_back();
            return back.first;
        }
    }
};


Object print() {return Object(Print);}
Object println() {return Object(Println);}

Object add() {return Object(Add);}
Object mul() {return Object(Mul);}
Object sub() {return Object(Sub);}
Object div() {return Object(Div);}
Object mod() {return Object(Mod);}

Object load() {return Object(Load);}
Object store() {return Object(Store);}
Object call() {return Object(Call);}
Object set_attr() {return Object(SetAttr);}
Object get_attr() {return Object(GetAttr);}

template<typename T>
Object value(T t) {return Object(t);}

Object function(vector<Object> t) {
    return Object(t);
}


int main() {
    // Object("hello world!").println();
    // Object(100).println();
    // Object().println();
    // (Object(19876545678876543150.1512) / Object(1)).println();
    // (Object(0) == Object(0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001)).println();
    
    // Object obj = Object();
    // obj.set_attr(Object(Object(-1)), "x");
    // obj.set_attr(Object(Object(1)), "y");
    // obj.set_attr(obj, "self");
    // obj.println();
    // obj.get_attr("self").println();
    // obj.get_attr("x").println();
    // obj.get_attr("y").println();

    StackFrame(Object(Object(
        {
            // Object(10), 
            // Object("test"), 
            // Object(Store),
            // Object({
            //     Object({
            //         Object("test"),
            //         Object(Load),
            //     }),
            //     Object(Call),
            // }),
            // Object(Call),
            // Object(Println),
            // Object(),
            // Object(200),
            // Object("x"),
            // Object(SetAttr),
            // Object(300),
            // Object("y"),
            // Object(SetAttr),
            // Object("obj"),
            // Object(Store),
            // Object("obj"),
            // Object(Load),
            // Object("x"),
            // Object(GetAttr),
            // Object("x: "),
            // Object(Print),
            // Object(Println),
            // Object("obj"),
            // Object(Load),
            // Object("y"),
            // Object(GetAttr),
            // Object("y: "),
            // Object(Print),
            // Object(Println),

            // Object("obj"),
            // Object(Load),
            // Object(500),
            // Object("y"),
            // Object(SetAttr),
            // Object("obj"),
            // Object(Store),

            // Object("obj"),
            // Object(Load),
            // Object("x"),
            // Object(GetAttr),
            // Object("x: "),
            // Object(Print),
            // Object(Println),
            // Object("obj"),
            // Object(Load),
            // Object("y"),
            // Object(GetAttr),
            // Object("y: "),
            // Object(Print),
            // Object(Println),


            // Object(5),
            // Object("a"),
            // Object(Store),
            // Object("a"),
            // Object(Load),
            // Object({
            //     Object("b"),
            //     Object(Store),
            //     Object(2),
            //     Object("b"),
            //     Object(Load),
            //     Object(Add),
            // }),
            // Object(Call),
            // Object("reult"),
            // Object(Store),
            // Object("reult"),
            // Object(Load),
            // Object(Println),



            // Object(3),
            // Object({
            //     Object("test"),
            //     Object(Store),
            //     Object({
            //         Object("test"),
            //         Object(Load),
            //         Object(Mul)
            //     })
            // }),
            // Object(Call),

            // Object("triple"),
            // Object(Store),

            // Object(2),
            // Object("triple"),
            // Object(Load),
            // Object(Call),

            // Object(Println),



            // Object({
            //     Object(Print),
            //     Object(Pass),
            // }),
            // function({
            //     println(),
            // }),
            // value("Print"),
            // store(),


            // value("Hello world!"),
            // value("Print"),
            // load(),
            // call(),
            // Object(),
            // Object(),
            // value("a"),
            // set_attr(),
            // value("obj"),
            // store(),

            // value("obj"),
            // load(),
            // value("obj"),
            // load(),
            // value("a"),
            // get_attr(),

            // value(200),
            // value("n"),
            // set_attr(),

            // value("a"),
            // set_attr(),
            // value("obj"),
            // store(),


            // value("obj"),
            // load(),
            // value("a"),
            // get_attr(),
            // value("n"),
            // get_attr(),
            // print(),

            // Object({
            //     Object("a"),
            //     Object(Store),
            //     Object("func"),
            //     Object(Store),

            //     Object("a"),
            //     Object(Load),
            //     Object("func"),
            //     Object(Load),
            //     Object(Call)
            // }),
            // Object("Test"),
            // Object(Store),


            // Object("Print"),
            // Object(Load),
            // Object("Hello functional programming!"),
            // Object("Test"),
            // Object(Load),
            // Object(Call),

            Object(),
            Object(),
            Object(),

            Object(2),
            value("c"),
            set_attr(),

            value("b"),
            set_attr(),

            value("a"),
            set_attr(),

            println(),

            

            Object(),
            value("p@$$w0rd"),
            Object("password"),
            set_attr(),

            value("adamthekiwi"),
            Object("username"),
            set_attr(),

            println(),

        }
    ))).run();

    return 0;
}
