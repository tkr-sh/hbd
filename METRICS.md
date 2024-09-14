This is the metrics for 1 million birthdays registered.

**Benchmark 1**: "hbd --help"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd --help` | 2.2 ± 0.6 | 0.2 | 3.6 | 1.00 |

**Benchmark 2**: "hbd add user1 06-06"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user1 06-06` | 86.6 ± 4.9 | 83.0 | 103.9 | 1.00 |

**Benchmark 3**: "hbd add user2 06-06"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user2 06-06` | 87.9 ± 3.4 | 83.3 | 97.8 | 1.00 |

**Benchmark 4**: "hbd add user2 06-07"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user2 06-07` | 46.6 ± 1.0 | 44.7 | 48.7 | 1.00 |

**Benchmark 5**: "hbd rename user2 user"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd rename user2 user` | 91.1 ± 24.8 | 82.7 | 187.0 | 1.00 |

**Benchmark 6**: "hbd list"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd list` | 378.1 ± 8.6 | 359.9 | 386.9 | 1.00 |

**Benchmark 7**: "hbd list -l 100"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd list -l 100` | 148.9 ± 0.9 | 146.8 | 150.3 | 1.00 |

**Benchmark 8**: "hbd list -L 100"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd list -L 100` | 61.5 ± 0.8 | 60.2 | 63.5 | 1.00 |

**Benchmark 9**: "hbd get"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd get` | 61.8 ± 0.6 | 60.8 | 63.0 | 1.00 |

