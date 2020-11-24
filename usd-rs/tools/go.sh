export PATH=/Volumes/src/cppmm/build

#cppmm /Volumes/src/usd-rs/usd-rs/tools/bind -u                                     \
cppmm ./bind -u                                     \
    -o usd-c                                        \
    --                                              \
    -I../../target/rls/debug/build/usd-cpp-297e9d6eb6be2ed0/out/install/include \
    -isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk \
    -isystem/Library/Developer/CommandLineTools/usr/include/c++/v1 \
    -isystem/Volumes/src/clang+llvm-11.0.0-x86_64-apple-darwin/lib/clang/11.0.0/include
    #-l ../../target/debug/deps/libusd.dylib         \
    #-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include \
