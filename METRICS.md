This is the metrics for 1 million birthdays registered.

**Benchmark 1**: "hbd --help"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd --help` | 2.4 ± 0.5 | 0.3 | 3.5 | 1.00 |

**Benchmark 2**: "hbd add user1 06-06"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user1 06-06` | 84.7 ± 1.8 | 82.3 | 88.1 | 1.00 |

**Benchmark 3**: "hbd add user2 06-06"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user2 06-06` | 85.2 ± 1.9 | 82.8 | 90.1 | 1.00 |

**Benchmark 4**: "hbd add user2 06-07"
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/hbd add user2 06-07` | 46.1 ± 0.8 | 44.2 | 48.0 | 1.00 |

**Benchmark 5**: " "
| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| ` ` | 265.7 ± 91.3 | 90.3 | 540.7 | 1.00 |

**Benchmark 6**: " "
| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| ` ` | 0.0 ± 0.0 | 0.0 | 0.0 | 1.00 |

**Benchmark 7**: " "
| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| ` ` | 9.5 ± 42.5 | 0.0 | 189.9 | 1.00 |

**Benchmark 8**: " "
| Command | Mean [µs] | Min [µs] | Max [µs] | Relative |
|:---|---:|---:|---:|---:|
| ` ` | 337.9 ± 105.6 | 153.0 | 771.6 | 1.00 |

**Benchmark 9**: ""

