#include<stdio.h>
#include<dlfcn.h>

void ShowError() {
  char *dlError = dlerror();
  if(dlError) printf("Error with dl function: %s\n", dlError);
}

int main() {
  void *SharedObjectFile;
  void (*SpecialPrintFunction)(char*);

  // Load the shared libary;
  SharedObjectFile = dlopen("./speciallib.so", RTLD_LAZY);
  ShowError();
  // Obtain the address of a function in the shared library.
  SpecialPrintFunction = dlsym(SharedObjectFile, "SpecialPrint");
  ShowError();

  printf("normal printf argument\n");
  // Use the dynamically loaded function.
  (*SpecialPrintFunction)("special print's argument");

  dlclose(SharedObjectFile);
  ShowError();
}
