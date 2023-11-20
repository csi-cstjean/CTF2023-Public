gcc ./src/main.c -m32 -static -Wl,-z,noseparate-code -Wno-implicit-function-declaration -I.. -no-pie -DNOPIE  -s -o challenge -fno-stack-protector
