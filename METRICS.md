This is the metrics for 1 million birthdays registered.

**Benchmark 1**: "hbd --help"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd --help` | 1.9 ± 0.4 | 0.5 | 3.0 | 1.00 |

**Benchmark 2**: "hbd add user1 06-06"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user1 06-06` | 84.2 ± 1.7 | 82.1 | 87.3 | 1.00 |

**Benchmark 3**: "hbd add user2 06-06"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user2 06-06` | 91.9 ± 28.4 | 82.6 | 198.3 | 1.00 |

**Benchmark 4**: "hbd add user2 06-07"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user2 06-07` | 47.6 ± 1.2 | 45.5 | 52.4 | 1.00 |

**Benchmark 5**: "hbd rename user2 user"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd rename user2 user` | 98.0 ± 27.2 | 81.4 | 187.9 | 1.00 |

**Benchmark 6**: "hbd list"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd list` | 160.2 ± 1.7 | 157.6 | 163.2 | 1.00 |

**Benchmark 7**: "hbd list -l 100"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd list -l 100` | 152.9 ± 1.2 | 151.0 | 155.5 | 1.00 |

**Benchmark 8**: "hbd list -L 100"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd list -L 100` | 62.5 ± 0.8 | 60.9 | 64.2 | 1.00 |

**Benchmark 9**: "hbd get"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd get` | 62.7 ± 0.8 | 61.2 | 65.1 | 1.00 |

