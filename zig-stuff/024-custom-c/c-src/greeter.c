#include "greeter.h"
#include <stdio.h>

void greet(const char *name) {
  if (name == NULL) {
    printf("Hello, world!\n");
  } else {
    printf("Hello, %s!\n", name);
  }
}
