#include <stdio.h>
#include <assert.h>
#include "../include/drain.h"
#include <pthread.h>
#include <string.h>

#define NUM_THREADS 4
#define LINES_PER_THREAD 1000

typedef struct {
    Drain *d;
    int    thread_id;
} thread_arg_t;

void *worker(void *arg) {
    thread_arg_t *t = (thread_arg_t *)arg;
    char line[256];
    
    for (int i = 0; i < LINES_PER_THREAD; i++) {
        // Alternate between two structurally identical lines
        if (i % 2 == 0) {
            snprintf(line, sizeof(line),
                "sshd[%d]: Failed password for user%d from 192.168.1.%d",
                i, t->thread_id, i % 255);
        } else {
            snprintf(line, sizeof(line),
                "kernel[%d]: Out of memory: killed process %d",
                i, i);
        }
        
        uint64_t id;
        char   **params;
        int      len;
        int rc = drain_parse(t->d, line, &id, &params, &len);
        if (rc != 0) {
            fprintf(stderr, "thread %d: parse failed\n", t->thread_id);
            return NULL;
        }
        drain_free_params(params, len);
    }
    return NULL;
}

int main(void) {
    Drain *d = drain_create(0.5);
    assert(d != NULL);

    pthread_t threads[NUM_THREADS];
    thread_arg_t args[NUM_THREADS];

    for (int i = 0; i < NUM_THREADS; i++) {
        args[i].d = d;
        args[i].thread_id = i;
        pthread_create(&threads[i], NULL, worker, &args[i]);
    }

    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    drain_destroy(d);
    puts("OK");
    return 0;
}
