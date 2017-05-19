// Clang or GCC
//   $ clang -Wall embed.c -L../target/release/ -lembed -o embed
//   $ gcc -Wall embed.c -L../target/release/ -lembed -o embed

extern void process();

int main(int argc, char **argv) {
    process(); // call Rust function
    return 0;
}
