cargo build
cd ..
./maelstrom/maelstrom test -w g-counter --bin whirlpool/target/debug/whirlpool -- g_counter --node-count 3 --rate 100 --time-limit 20 --nemesis partition