cargo build
cd ..
./maelstrom/maelstrom test -w echo --bin whirlpool/target/debug/whirlpool -- echo --node-count 1 --time-limit 10