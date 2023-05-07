cargo build
cd ..
./maelstrom/maelstrom test -w broadcast --bin whirlpool/target/debug/whirlpool -- broadcast2 --node-count 5 --time-limit 20 --rate 10 --nemesis partition
# This one succeded with with the broadcast2 implementation.