cargo build
cd ..
./maelstrom/maelstrom test -w broadcast --bin whirlpool/target/debug/whirlpool -- broadcast --node-count 1 --time-limit 20 --rate 10
#This somehow fails even if I dont see any failing tests in the logs, and broacast2 passes.