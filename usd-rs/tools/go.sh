export PATH=/Volumes/src/cppmm/build

cppmm ./bind -u                                     \
    -l ../../target/debug/deps/libusd.dylib         \
    -o usd-c                                        \
    --                                              \
    -I../thirdparty/docs/include                    \
    -isystem /Volumes/src/clang+llvm-11.0.0-x86_64-apple-darwin/lib/clang/11.0.0/include
