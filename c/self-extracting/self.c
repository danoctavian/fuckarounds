#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>

unsigned char hello_world[] = {
0x50,0x57,0x56,0x52,0xe8,
0x00,0x00,0x00,0x00,0x5e,
0x48,0x81,0xc6,0x24,0x00,
0x00,0x00,0x48,0xc7,0xc0,
0x01,0x00,0x00,0x00,0x48,
0xc7,0xc7,0x01,0x00,0x00,
0x00,0x48,0xc7,0xc2,0x0e,
0x00,0x00,0x00,0x0f,0x05,
0x5a,0x5e,0x5f,0x5a,0xc3,
0x48,0x65,0x6c,0x6c,0x6f,
0x2c,0x20,0x57,0x6f,0x72,
0x6c,0x64,0x21,0x0a,0x00
};
int hello_size = 60;
void* get_page(void* addr) {
  return (void*)((unsigned long)addr & ((0UL - 1UL) ^ 0xfff));
}

void make_exec_mem(void* addr) {
  void* page = get_page(addr);
   int ans = mprotect(page, 1, PROT_READ|PROT_WRITE|PROT_EXEC);/*set page attributes*/
  if (ans) {
    perror("mprotect");
    exit(EXIT_FAILURE);
 } 
}

void* mmap_code_file(char* file_name) {
  int fd = open("alph", O_RDONLY, S_IREAD);
  
  struct stat buf;
  fstat(fd, &buf);
  int size = buf.st_size;
  printf("file size %d of file %d \n", size, fd);
  void* map = mmap(0, size, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0);
  if (map == MAP_FAILED) {
   close(fd);
   perror("Error mmapping the file");
   exit(EXIT_FAILURE);
  } 
  close(fd);
  return map;
}

int main(int argc, char**argv) {

  make_exec_mem(hello_world);
  ((void(*)(void))hello_world)();

  // execute from heap as well
  unsigned char* heap_code = malloc(hello_size * sizeof(unsigned char));  
  int i;
  for (i = 0; i < hello_size; i++) {
    heap_code[i] = hello_world[i];
  }
 
  make_exec_mem(heap_code); 
  ((void(*)(void))heap_code)();
  free(heap_code);
  void* file = mmap_code_file("alph");
  printf("the first char is %d %c \n", 12, *((int*) file));
 return 0;
}
