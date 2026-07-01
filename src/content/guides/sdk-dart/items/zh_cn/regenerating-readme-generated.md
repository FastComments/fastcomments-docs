---
生成器会在本地运行的 FastComments 服务器 (`http://localhost:3001/js/swagger.json`) 可用时获取规范，若不可用则回退到已提交的 `openapi.json`.

```bash
python3 update.py
```

需要 `node`/`npx`（用于 `@openapitools/openapi-generator-cli`）和 Java.
---