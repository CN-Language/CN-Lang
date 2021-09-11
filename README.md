# CN lang design draft

## Documentation
```
// if you set lang, all this file doc is at lang's format
//!lang=MarkDown(Default)/Org/Wiki/Html/Rst/AsciiDoc(Custom="executable path")
//!from="../README.MD"

//! #header
//! [link](www.google.com)

/// blablabla
imports and defs
```
finally will generate HTML

## Variable Shallowing
```
int a = 10;
char* a = "a";
char[] a = @meta @raii{
    File* f = new File(a);
    @emit(StringLiteral(f.String()))
};

int main(){
    int res = 10;
    char* res = f(res);
    int res = g(res);
    return res;
}
```

## RAII hooks
- `Ref::new`
- `Ref::delete`
- `Value::init`
- `Value::destroy`
- `Value::default` operator is use when `{default}` literal is used.

all this functions has default implementation.

- `new`: malloc + init
- `delete`: destroy + free
- `init`: 0
- `destroy`: do nothing
- `default`: init

## RAII block
because of the goal of CNlang is not a higher-level function, CNlang does not want to modify the semantic of C, so RAII is optional
```
@raii{

}
```

## Expr block
the last value of expr block is used as the expr, is alternative to the `gcc` extension.
```
@expr if
@expr switch
@expr match
@expr {}
// no need return clause
void func() @expr {

}
// similar to rust semantic
char* read_10(char* path) @expr @raii {
    File f = new File{path:path};
    char* ch = new char[10];
    fgets(ch, 10, f);

    ch
    // f.close is called automatically
}
```

## distructors
```cn
struct A{int a};

int main(){
    A a = {a:5};
    A{int b} = a;
    @assert(b == 5);
    &A{int b_ptr} = &a;
    @assert(b_ptr = &(a->a));
    *A{int b} = *a;
    @assert(b = 5);
}
```

## error handling
require runtime feature
- throws
- try
- catch
- continue in catch
- break in catch
- throw

**this is only for unchecked exception**

```cn
void lots_of_exception() @throws(Exception1, Exception2) {
    try{
        for(;;){
            new char[114514];
            some_other_function();
            throw DUM;
        }
    }
    catch(OOM oom){
        abort();
    }
    catch(DUM dum){
        println(dum.stackTrace().String());
        continue;
    }
}
```

## checked exception
- interface `Exception`

```
int main(){
    @expr @raii @checked_exception_cps({
        err1: int a = f()?;
        err2: int b = g(a)?;
        err3: int c = h(b)?;
        Ok(c)
    },(err1){
        Err("1")
    },(err2){
        Err("2")
    },(err3){
        Err("3")
    });
}
```

## match **statement** and distructors (SOME FP GUYS REALLY WANT TO KILL ME)

## inline structures
DISCUSS: is that considered inheritance?

```cn
struct Parent{
    const char* name;
}
struct Child{
    inline Parent parent;
    // still aba-abaing
}
int main(){
    Child child = {name: "David"};
    child.name;
}
```
not allowed because of we are not cpp:
```cn
struct Parent{
    const char* name;
}
struct Child{
    inline Parent parent;
    // compiler error
    const char* name;
}
```
but we do allow `Child` to call UFC  works for `Parent`

## tunion(AKA tagged union)
TODO: rename?

TODO: change syntax?

TODO: replace enum?

## row poly

```cn
struct{int a; int b;|} s;
union{int a; double b;|} u;
tunion{int a; double b;|} tu;
```

```cn
// row poly generic is treat like template
void f(struct{int a;|}* s);

template<struct{int a;|} T>
void f(T* s);

// dynamic dispatch
void f(struct{int a;|}^ s);
```

## interface
```cn
interface Name{
    void function_def(Ty,Ty);
}
// interface pointer/ value will be treat like template
void function(Name* name);

// equiv
template<Type T{Interface}>
void function(T* name);

// dynamic dispatch
void function(Name^ name);
```

## dynamic dispatch(i know what's your desire)
DRAFT:
require runtime feature
- `^` means dynamic pointer with vtable ptr
- `Interface^`
- `RowPolyGeneric^`

vptr + vtable + value ptr

## runtime reflection
require runtime feature with runtime reflection

DISCUSS: binary will contain meta data and compile speed slow down and compile size increase and useless 

TODO

## dynamic function(which is method missing able)
- dyn_call
- register
- on_miss

DISCUSS: do we really need that? is so fucking slow! and we need to write a quite handy runtime for that.overlaps with template.

looks like OBJC and works like objc

```cn
int main(){
    int a = 1;
    a.@dyn_call(int add(int,int,int))(2,3);
}
// in some other module
int add23(int a, int b, int c){
    return a + b + c;
}

// expect to work
template<Type T>
T add23(T t){
    try {
        t.@dyn_call(T add(T,T,T))(T{2},T{3});
    }catch(Err^ err){
        if (err.string() == "MethodMissing"){
            bool res = @register("some_module.o");
            if (res)
                continue;
            else
                throw Err::MethodNotFount{name: "add"};
        }
    }
}
```

## GC feature
- @gc stmt with new
- @nostw block

TODO

## module
- adding modules to build.cn
- filename as module
- dir as module name
  - mod.cn as the root module
  - all files belongs to the dir is part of the module and their file name will be a module
  - `module::submodule::var`

## building system
```cn
name = "project_name";
authors = {"name <email@email.com>"};
repo = "";
license = "license/filedir";

feature_list = {};

void build(Builder* builder){
    if (builder.TargetOS == "__LINUX__"){
        builder.define("__LINUX__");
    }
}

int run(Env env){
    if (env.exe == "main"){
        for (char* tag: env.tags){
            match (tag){
                case "-O3":
                case "-O2":
                default:
            }
        }
    }else{
        match (env.exe){
            case "install":
            case "uninstall":
            default:
                @panic(@format("unkownen command: {}", env.exe));
        }
    }
}
```

## Stable ABI
- @no_mangle
- @to_symbol
```cn
// project Hello
// module Hello
void hello(){
    // ...
}
```
with mangle: `MS_Hello_ME_hello`
with no_mangle will emit both `hello` and the prev
```cn
int main(){
    Ty ty = {};
    A a = {};
    B b = {};
    (&ty).acn(a);
    (&ty).acn(b);
}
```

## operator overloading
- @operator
```cn
Struct Mat4x4 {int[16] elem};
@operator Mat4x4 Mat4x4_OPADD_Mat4x4_Mat4x4 (Mat4x4 a, Mat4x4 b){
    Mat4x4 res = {};
    res[0] = a[0] + b[0];
    // ...
    return res;
}
int main(Env* env){
    Mat4x4 a = {elem: {
        1,1,1,1,
        1,1,1,1,
        1,1,1,1,
        1,1,1,1
    }};
    Mat4x4 b = {elem: {
        2,2,2,2,
        2,2,2,2,
        2,2,2,2,
        2,2,2,2
    }};
    Mat4x4 c = a + b;
}
```

## Macro
- @emit
- @emit_raw
- @write
- @quote
- @unquote
- @attribute_macro
- @macro

```cn
@JSON
struct SomeJson {
    int id;
    string name;
}

@attribute_macro JSONObject* JSON(AST* ast){
    match (ast.type){
        case "struct":
            FunctionBuilder fb = {};
            fb.ret_type = "JSON*";
            fb.name = @format("{}_toJson",ast.type);
            fb.args = {Args{ty: ast.type, name: arg}};
            fb.body = @quote {
                @unquote {JSONObject object = new JSONObject{};}
                for (field* : ast.fields){
                    @unquote {
                        object.add(field.toJsonField());
                    }
                }
                @unquote {return object;}
            };
            @emit(fb)
        default:
            @compiler_error("not supported");
    }
}

int main(){
    SomeJson sj = {};
    sj.toJson();
}
```

```cn
@macro call_default(Type $1, Function $2){
    @emit {
            @quote {
                // default initializer
                $1.name _1 = {};
                @meta {
                    if ($2.arg_count == 1 && $2.arg_type[0] == $1){
                        @emit { $2(_1) }
                    } else {
                        @compiler_error(@format("{} is not supported",$2.name))
                    }
                }
        }
    }
}

int main(){
    // yes it allows
    int inc(int a){return ++a;};
    @call_default(int,inc);
}
```

## compile time execution
- @meta

`const` is shared between compile time execution and runtime
`const` in compile time is mutable
`cconst` in compile time is immutable
compile time could include everything except something is dependent with this file
import also works with `@meta`
`@meta import` does not work with runtime

```cn
import stdio;
const version = 5;
@meta println(@format("version: {}",version));
int[] arr = {
    @meta for(int i = 1; i <= version; ++i) {
        @emit_raw(i)
        @write(",")
    }
};

int main(Env* env){
    @meta println("hello from compile time");
    println("hello from runtime);
    for(int i:arr){
        println(@format("{}",i));
    }
}
```
```
> version: 5
> hello from compile time
> hello from runtime
> 1
> 2
> 3
> 4
> 5
```

## compile time priority(so compile speed is slow)
- macro expansion
  - if inner is a macro `call` macro expansion to inner first
  - run emit/write
  - run unquote
  - run quote

- compile
  - look up cache of imports
    - compile if miss
  - parse to ast exclude codes in meta
  - expand template
  - run code in meta
    - compile code in meta
      - load parent context
        - template arguments
        - ast
        - imports
      - if has meta in meta `goto` run code in meta
    - execute code in meta
  - run attr macro
  - run macro expansion
  - reload
  - compile hole file

## template + UFC with stable ABI
`foreach` semantic
and only used template are exportedby default

- export_template

```cn
struct A<Type Ty>{Ty a};

// A<int> == MS_MODULE_ME_A_GB_int_EB

struct Tuple<Type[] Tys>{
    @meta {
        int count = 0;
        for(Ty: Tys){
            @write(@format("{} _{};",Ty, count));
            ++count;
        }
    }
}

/* 
struct Tuple<int,double,const char*>{
    int _0;
    double _1;
    const char* _2;
}
*/

struct SmallVec<Type T,int n>{
    int len;
    tunion{
        T[n] inline;
        T* heap;
    } body;
}

void add<Type T,int n>(SmallVec<T,n>* v, T e){
    if(v->len + 1> n){
        // malloc memcpy add
    }else{
        v.body.inline[v->len - 1] = e;
    }
    v->len++;
}
```

export unused template
```
template<Type T>
void f(T t){}

@export_template(f<int>,f<float>,f<double>)
```

## Compile time constant expansion
TODO: hard work on analysing where to execute

- compile_time
```cn
@compile_time int fib(int a){
    if(a==0||a==1)
        return 1;
    else{
        return fib(a-1) + fib(a-2);
    }
}
int main(){
    // 1 1 2 3 5 8 13
    const int a = fib(4);
    // int a = 5;
    int b = fib(a);
    // int b = 8;
}
```

## auto type
- auto

```
auto function_has_call_back(int a){
    return clos int (int b){
        return a + b;
    };
}

// auto will be replaced by `Closure<int(*)(int)>`
```

## overloading

```cn
int add(int a, int b){
    return a + b;
}

int add(int a, int b, int c){
    return a + b + c;
}
```

## lambda and closure
TODO: closure context save

lambda is just a anonymous function
```cn
auto lam = void(){println("hello")};
// void _lam_{__LINE__}_{__FUNC__}
```
type is `void(*)()`
```cn
void func(){
    int a;
    auto clos = clos void(){
        a += 1;
    }
}
```
type is `Closure<void(*)()>`
closure is never appared as a value

### Generic function pointer interface
- `OP_CALL<T>`


## thread local
- thread_local

```cn
thread_local int a = 0;
int main(){
    auto lam = void(){for(;;){
        a += 1;
        println(@format("{}",a));
    }};
    auto _ = spawn(lam);
    lam();
}
/*
1
2
3
1
4
2
3
5
...
*/
```

## C intergration + C export + General FFI interface
export module as a header
export ABI
C intergration
```cn
@fromC("src/a.c") void function(){}
```
first precompile every sub module
then generate header for hole project
```c
#include <stdio.h>
#include "cnabi.h"
#include "<$PROJECT_NAME>.h"

CNFUNCTIONDEF("void MODULE::function()"){
    CNTYPE("MODULE::TYPE<TYPE>") var = 
        CNFUNLOOKUP("MODULE::TYPE<TYPE> new(int,int)")(1,2,3);
    CNTYPE("MODULE::TYPE<TYPE>") var2 = 
        CNFUNLOOKUP("MODULE::TYPE<TYPE> MODULE::ADD(MODULE::TYPE<TYPE>,MODULE::TYPE<TYPE>)")(var,var);
    char* format = 
        CNFUN(char* into(String))
            (CNFUN("String String(MODULE::TYPE<TYPE>*)")
                (var2));
    printf("%s\n",format);

// some C only extension
    char* lab = &&label;
    goto *label;
label:
// for sure you can write inline asm in C
    __asm__((
        mov %eax %ebx
    ));
}
```

## Coroutine
stackful coroutine is implemented for async
the stack of coroutine is much more smaller than the normal stack for thread
so coroutine is just for concurrency

```
@coroutine void socket_listener(Socket *s) -> String{
    for(;;){
        auto res = await s.next();
        if(res != close){
            yield res.String()
        }else{
            return;
        }
    }
}
```

## better than good core std interfaces design
- runtime.allocator
- runtime.gc
- runtime.dynamic
- runtime.logger
- runtime.error_handler

- runtime.formatter

- atomicT
- option
- result

interfaces
- show
- eq
- partialeq
- ord
- partialord
- incable
- decable
- reversable
- hasher
- iter