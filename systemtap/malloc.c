#!/usr/bin/stap

global malloc_count

probe process("/usr/sbin/sshd").library("/usr/lib64/libc-2.32.so").function("malloc") {
  malloc_count++
}

probe timer.s(1) {
  printf("malloc_count:%d\n", malloc_count)
}
