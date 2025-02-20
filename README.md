# simple-todo-list

## 実行方法
```
cargo run
```


### タスクを追加
```
curl -X POST "http://localhost:3000/tasks" -H "Content-Type: application/json" -d '{"title": "Learn Rust"}'
```


### タスク一覧を取得
```
curl -X GET "http://localhost:3000/tasks"
```

### タスクを削除
```
curl -X DELETE "http://localhost:3000/tasks/<task_id>"
```