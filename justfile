t contest task:
    rm -r test
    oj download https://atcoder.jp/contests/{{contest}}/tasks/{{task}}
    oj t -c "cargo run --bin {{contest}}_{{task}}"

