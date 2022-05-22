# To do list command-line

## help message
```bash
cargo run -- help
```

## Add task (string)
```bash
cargo run -- -j [filename.json] add "to do message" 
```
ex)
```bash 
cargo run -- -j todays-task.json add "buy eggs"
```

## Delete task (task ID)  
task ID: Check the list
```bash
cargo run -- -j [filename.json] done [ID]
```
ex)
```bash
cargo run -- -j todays-task.json done 2
```

## Task list
```bash
cargo run -- -j [filename.json] list
```
ex)
```bash
cargo run -- -j todays-task.json list
1: take a walk            [2022-05-22 18:31]
2: buy eggs               [2022-05-22 18:33]
       :
```

<br>

## todo-list.exe
no need cargo
```bash
./todo-list.exe -j [filename.json] add "task"
./todo-list.exe -j [filename.json] done ID
./todo-list.exe -j [filename.json] list
```