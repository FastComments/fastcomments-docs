---
Генератор берёт спецификацию с локально запущенного сервера FastComments  
(`http://localhost:3001/js/swagger.json`), если она доступна, иначе использует  
коммитнутый `openapi.json`.

```bash
python3 update.py
```

Требуется `node`/`npx` (для `@openapitools/openapi-generator-cli`) и Java.
---