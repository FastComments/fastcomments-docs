---
Генератор получает спецификацию с локально запущенного сервера FastComments
(`http://localhost:3001/js/swagger.json`) когда доступен, в противном случае
использует закоммиченный `openapi.json`.

```bash
python3 update.py
```

Требуется `node`/`npx` (для `@openapitools/openapi-generator-cli`) и Java.
---