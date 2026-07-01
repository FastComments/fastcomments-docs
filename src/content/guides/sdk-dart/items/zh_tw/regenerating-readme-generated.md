---
生成器會從本機執行的 FastComments 伺服器（`http://localhost:3001/js/swagger.json`）取得規格，若可取得，否則會回退至已提交的 `openapi.json`。

```bash
python3 update.py
```

需要 `node`/`npx`（用於 `@openapitools/openapi-generator-cli`）以及 Java。
---