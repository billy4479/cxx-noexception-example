#!/bin/sh
g++ \
    main.cpp \
    target/cxxbridge/cxx-noexception-example/src/lib.rs.cc \
    target/debug/libcxx_noexception_example.a \
    -DRUST_CXX_NO_EXCEPTIONS \
    -fno-exceptions \
    -fpermissive \
    -o cxx
