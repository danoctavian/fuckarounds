#include <sys/stat.h>
#include <sys/types.h>
#include <stdio.h>
#include <fcntl.h>


void main() {
  int fd = open("alph", O_RDONLY, S_IREAD);
  struct stat buf;
  fstat(fd, &buf);
  int size = buf.st_size;
  printf("file size %d of file %d \n", size, fd);
  close(fd);
}
