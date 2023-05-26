#include <stdio.h>

typedef struct {
  char *fname, *lname;
  int age;
} Person;

Person person_create(char *fname, char *lname, int age) {
  Person person;
  person.fname = fname;
  person.lname = lname;
  person.age = age;
  return person;
}

void person_print(Person *person) {
  printf("First name: %s\n", person->fname);
  printf("Last name: %s\n", person->lname);
  printf("Age: %d\n", person->age);
}

int main(void) {
  Person person = person_create("John", "Doe", 32);
  person_print(&person);
  return 0;
}
