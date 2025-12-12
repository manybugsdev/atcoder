t contest task:
    touch ./src/bin/{{contest}}_{{task}}.rs
    rm -rf test
    oj download https://atcoder.jp/contests/{{contest}}/tasks/{{task}}
    oj t -c "cargo run --bin {{contest}}_{{task}}"
