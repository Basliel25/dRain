#include <stdio.h>
#include <assert.h>
#include "../include/drain.h"

int main(void) {
    Drain *d = drain_create(0.5);
    assert(d != NULL);

    uint64_t id;
    char   **params;
    int      len;

    // dummy line
    int rc = drain_parse(d,
        "sshd[1234]: Failed password for alice from 192.168.1.1",
        &id, &params, &len);
    assert(rc == 0);
    printf("template_id=%llu params=%d\n", (unsigned long long)id, len);
    drain_free_params(params, len);

    rc = drain_parse(d,
        "sshd[5678]: Failed password for bob from 10.0.0.1",
        &id, &params, &len);
    assert(rc == 0);
    printf("template_id=%llu params=%d\n", (unsigned long long)id, len);
    for (int i = 0; i < len; i++) printf("  param[%d]=%s\n", i, params[i]);
    drain_free_params(params, len);

    drain_destroy(d);
    puts("OK");
    return 0;
}
