#!/bin/sh
cd tests
make > /dev/null
./mandel_integration_tests
make clean > /dev/null
cd ..
