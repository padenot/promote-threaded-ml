# promote-threaded-ml

`threaded-ml` threads aren't promoted to real-time in the `libpulse` I've found
and this causes audio glitches when the machine is under load (e.g. building a
large software project).

This program, adapted from
[`audio_thread_priority`](https://github.com/padenot/audio_thread_priority/)
can promote any thread using its `pid` and `tid` (kernel task id, not pthread_t), like so:

```sh
$ ./promote-threaded-ml pid tid
```

An included shell script promotes all threads matching a pattern (here `threaded-ml`):

```sh
$ ./promote-all-threaded-ml.sh
```

Sample run:

```sh
$ ps -eL -o s,pid,tid,cls,pri,comm | grep "threaded-ml"
S    2896   10229  TS  19 threaded-ml
S    5651    5684  TS  19 threaded-ml
S 1956201 1956359  TS  50 threaded-ml
S 1956201 1956789  TS  50 threaded-ml
S 1956201 1957874  TS  50 threaded-ml
S 2460191 2460665  TS  19 threaded-ml
S 2474733 2474971  TS  50 threaded-ml
S 2474733 2475463  TS  50 threaded-ml

$ ./promote-all-threaded-ml.sh
<garbage output useful to diagnose>

$ ps -eL -o s,pid,tid,cls,pri,comm | grep "threaded-ml"
S    2896   10229  TS  19 threaded-ml
S    5651    5684  TS  19 threaded-ml
S 1956201 1956359  RR  50 threaded-ml
S 1956201 1956789  RR  50 threaded-ml
S 1956201 1957874  RR  50 threaded-ml
S 2460191 2460665  TS  19 threaded-ml
S 2474733 2474971  RR  50 threaded-ml
S 2474733 2475463  RR  50 threaded-ml
```

# License

MLP 2.0