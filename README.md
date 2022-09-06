TODO create 10 pingers pinned to different threads
TODO monoio




```
cargo build --all --release
   Compiling pingo_pongo v0.1.0 (/home/dr/src/pingo_pongo)
    Finished release [optimized] target(s) in 2.85s
target/release/glommio
Listening on 127.0.0.1:8001
Connected to 127.0.0.1:8001
elapsed ponger 144.058876869s per iteration 72.029µs per second 13883.212499419255
elapsed pinger 144.059036246s per iteration 72.029µs per second 13883.19713998873
target/release/glommio
Listening on 127.0.0.1:8001
Connected to 127.0.0.1:8001
elapsed ponger 143.103649841s per iteration 71.551µs per second 13975.883929041402
elapsed pinger 143.103770357s per iteration 71.551µs per second 13975.872159137482
target/release/glommio
Listening on 127.0.0.1:8001
Connected to 127.0.0.1:8001
elapsed ponger 145.045981221s per iteration 72.522µs per second 13788.730878056458
elapsed pinger 145.046072992s per iteration 72.523µs per second 13788.722153893195
target/release/glommio
Listening on 127.0.0.1:8001
Connected to 127.0.0.1:8001
elapsed ponger 144.299377878s per iteration 72.149µs per second 13860.073615084668
elapsed pinger 144.299491891s per iteration 72.149µs per second 13860.062664051144
target/release/glommio
Listening on 127.0.0.1:8001
Connected to 127.0.0.1:8001
elapsed ponger 145.442459648s per iteration 72.721µs per second 13751.142581336991
elapsed pinger 145.442526853s per iteration 72.721µs per second 13751.136227311405

target/release/tokio
Listening on 127.0.0.1:8002
Connected to 127.0.0.1:8002
elapsed ponger 82.639113356s per iteration 41.319µs per second 24201.614934858088
elapsed pinger 82.63939925s per iteration 41.319µs per second 24201.531208493147
target/release/tokio
Listening on 127.0.0.1:8002
Connected to 127.0.0.1:8002
elapsed ponger 82.494722534s per iteration 41.247µs per second 24243.975112174052
elapsed pinger 82.494790762s per iteration 41.247µs per second 24243.955060993627
target/release/tokio
Listening on 127.0.0.1:8002
Connected to 127.0.0.1:8002
elapsed ponger 83.348075486s per iteration 41.674µs per second 23995.75501099531
elapsed pinger 83.348121692s per iteration 41.674µs per second 23995.741708381724
target/release/tokio
Listening on 127.0.0.1:8002
Connected to 127.0.0.1:8002
elapsed ponger 81.921790769s per iteration 40.96µs per second 24413.528820915613
elapsed pinger 81.921862263s per iteration 40.96µs per second 24413.507514993344
target/release/tokio
Listening on 127.0.0.1:8002
Connected to 127.0.0.1:8002
elapsed ponger 82.438844339s per iteration 41.219µs per second 24260.408015616056
elapsed pinger 82.438877632s per iteration 41.219µs per second 24260.39821803284

target/release/tokio_uring 
Listening on 127.0.0.1:8003
elapsed ponger 111.941157936s per iteration 55.97µs per second 17866.52949528589
elapsed pinger 111.941263282s per iteration 55.97µs per second 17866.512681401884
target/release/tokio_uring 
Listening on 127.0.0.1:8003
elapsed ponger 111.43179347s per iteration 55.715µs per second 17948.198962968734
elapsed pinger 111.431892986s per iteration 55.715µs per second 17948.182934048105
target/release/tokio_uring 
Listening on 127.0.0.1:8003
elapsed ponger 111.776885162s per iteration 55.888µs per second 17892.78702033402
elapsed pinger 111.777004767s per iteration 55.888µs per second 17892.767874474852
target/release/tokio_uring 
Listening on 127.0.0.1:8003
elapsed ponger 112.005504592s per iteration 56.002µs per second 17856.265254867216
elapsed pinger 112.005559545s per iteration 56.002µs per second 17856.256494093657
target/release/tokio_uring 
Listening on 127.0.0.1:8003
elapsed ponger 112.741849882s per iteration 56.37µs per second 17739.641509282294
elapsed pinger 112.741981337s per iteration 56.37µs per second 17739.620825198625

target/release/std_net
Listening on 127.0.0.1:8003
Connected to 127.0.0.1:8003
elapsed ponger 67.960026577s per iteration 33.98µs per second 29429.064418242426
elapsed pinger 67.960029222s per iteration 33.98µs per second 29429.06327286511
target/release/std_net
Listening on 127.0.0.1:8003
Connected to 127.0.0.1:8003
elapsed pinger 69.030537391s per iteration 34.515µs per second 28972.684779660347
elapsed ponger 69.030535998s per iteration 34.515µs per second 28972.685364313922
target/release/std_net
Listening on 127.0.0.1:8003
Connected to 127.0.0.1:8003
elapsed ponger 68.703311321s per iteration 34.351µs per second 29110.678387180964
elapsed pinger 68.703323414s per iteration 34.351µs per second 29110.673263186723
target/release/std_net
Listening on 127.0.0.1:8003
Connected to 127.0.0.1:8003
elapsed pinger 69.389293406s per iteration 34.694µs per second 28822.890417660066
elapsed ponger 69.38929546s per iteration 34.694µs per second 28822.889564470584
target/release/std_net
Listening on 127.0.0.1:8003
Connected to 127.0.0.1:8003
elapsed pinger 70.595668124s per iteration 35.297µs per second 28330.35019212562
elapsed ponger 70.595662695s per iteration 35.297µs per second 28330.352370807217



target/release/monoio               
Listening on 127.0.0.1:8001
elapsed ponger 85.549247518s per iteration 42.774µs per second 23378.347069378837
elapsed pinger 85.549412226s per iteration 42.774µs per second 23378.302059124657


```


