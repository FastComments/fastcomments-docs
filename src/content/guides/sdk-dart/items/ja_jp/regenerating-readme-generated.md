---
ジェネレータは、利用可能な場合はローカルで実行中の FastComments サーバー (`http://localhost:3001/js/swagger.json`) から仕様を取得し、利用できない場合はコミットされた `openapi.json` にフォールバックします。

```bash
python3 update.py
```

`node`/`npx`（`@openapitools/openapi-generator-cli` 用） と Java が必要です。
---