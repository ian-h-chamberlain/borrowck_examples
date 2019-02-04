#include "borrow.hpp"

void
example_1()
{
    int foo = 3;

    auto& bar = foo;

    {
        auto& baz = foo;

        baz = 47;
    }

    bar += 1;

    std::cout << "Example 1: " << foo << std::endl;
}

void
example_2()
{
    std::string my_string = "hello";

    std::string& ref1 = my_string;

    const std::string& ref2 = my_string;

    ref1 = "foo";

    // ref2's value changed from underneath it!
    std::cout << "Example 2: " << my_string << ", " << ref2 << std::endl;
}

void
example_3()
{
    std::string my_string = "hello";

    const std::string& ref1 = my_string;
    const std::string& ref2 = my_string;

    std::string& r3 = my_string;

    r3 = "foo";

    // Even though they were const, ref1 and ref2 values changed!
    std::cout << "Example 3: " << ref1 << ", " << ref2 << std::endl;
}

void
example_4()
{
    // can't use references for this example since they have to be initialized
    std::string* ref1;
    {
        std::string my_string = "hello";
        ref1 = &my_string;
    }

    // UNDEFINED BEHAVIOR! It may or may not do what we expect
    *ref1 = "my fair lady";

    std::cout << "Example 4: " << *ref1 << std::endl;
}
