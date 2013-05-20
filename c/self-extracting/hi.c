#include <stdio.h>
#include <stdlib.h>

int ret_1() {
  return 1337;
}

void say_hi() {
  printf("i'm saying hi !\n");
}

int main() {
  say_hi();
  printf("shit son %d\n", ret_1());
  return 0;
}
