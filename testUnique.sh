cargo build
cd ..
./maelstrom/maelstrom test -w unique-ids --bin whirlpool/target/debug/whirlpool -- unique --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition