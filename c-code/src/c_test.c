/*
This i a few c-function that return different values used as examples of the Rust c ffi functionality 
*/
#include<stdio.h>
#include<string.h>
int *return_pointer(int *, int); // this function returns a pointer of type int
int *no_arg(); //returns int pointer
struct test_struct no_arg_struct(); //returns a struct 
struct test_struct *no_arg_struct_p(); // returna a struct pointer 

struct test_struct {
   int32_t    x;
   int    y;
   int32_t    z;
};

int *no_arg()
{
    int i;
    i = 4;
    int *j = &i;
    return &i;
}

struct test_struct no_arg_struct()
{
    struct test_struct p = { 666,7727,2 };
    return p;
}

struct test_struct *no_arg_struct_p()
{
    struct test_struct p = { 666,7727,2 };
    return &p;
}
