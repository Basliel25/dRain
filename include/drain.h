#pragma once
#include <stdint.h>

typedef struct Drain Drain;

Drain *drain_create(double threshold);

void drain_destroy(Drain *handle);

/**
 * Returns 0 on success, -1 on failure.
 * params_out is heap-allocated caller needs to free, drain has a helper function 
 */
int drain_parse(
    Drain          *handle,
    const char     *raw_line,
    uint64_t       *template_id_out,
    char         ***params_out,
    int            *params_len_out
);

/** Free params array returned by drain_parse. */
void drain_free_params(char **params, int len);
