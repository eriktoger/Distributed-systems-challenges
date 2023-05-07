cargo build
cd ..
./maelstrom/maelstrom test -w broadcast --bin whirlpool/target/debug/whirlpool -- broadcast2 --node-count 25 --time-limit 20 --rate 100 --latency 100
# This one succeded with with the broadcast2 implementation.

# maybe I can just have the counter as state?
#https://github.com/sak96/gossip_glomers/blob/main/src/bin/g_counter.rs