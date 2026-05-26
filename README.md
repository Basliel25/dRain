# dRain

Streaming log template extraction in Rust, with a C FFI integration. 
It implements the Drain algorithm (He et al., ICWS 2017)[https://jiemingzhu.github.io/pub/pjhe_icws2017.pdf] with a fixed depth tree.

Read how I made it [here](https://basz-website.basgug25.workers.dev/projects/drain/)
**Integrates:** [trafilo](https://github.com/Basliel25/trafilo) for event transport


## Use

### As a stdin binary

```bash
cargo build --release
echo "sshd[1234]: Failed password for alice from 192.168.1.1" | \
    ./target/release/drain
# 0
```
output Format: `template_id<TAB>param1<TAB>param2...`

### As a C library

```c
#include <drain.h>

Drain *d = drain_create(0.5);  // similarity threshold

uint64_t id;
char   **params;
int      len;
drain_parse(d, "sshd[1234]: Failed password for alice", &id, &params, &len);
// id = 0
drain_free_params(params, len);

drain_destroy(d);
```

Build: `cargo build --release` produces `target/release/libdRain.so`.
Header: `include/drain.h`.

## Validation

| Corpus | Templates found | Drain3 ground truth | Status |
|---|---|---|---|
| Linux_2k.log | 34 | ~30 | ✅ within ±20% |
| OpenSSH_2k.log | 19 | ~50 | ⚠️ over-merging, see issue #5 |

## Known issues

- **Helgrind false positives** (regex crate) — internal lazy DFA
  construction triggers crazy data race reports.own code is race-free.

## Roadmap

- v0.1.1: persistence (save/load template table)
- v0.2: clean Rust crate API for publication to crates.io 
- Adaptive-depth tree (thesis extension)

## License
MIT
