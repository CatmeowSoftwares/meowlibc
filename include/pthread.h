#include <sys/types.h>


int pthread_cancel(pthread_t thread);

int pthread_create(pthread_t *pthread,
                   const pthread_attr_t *attr,
                   const void *start_routine,
                   const void *arg);

void pthread_exit(void *value_ptr);

int pthread_join(pthread_t thread, void **value_ptr);
