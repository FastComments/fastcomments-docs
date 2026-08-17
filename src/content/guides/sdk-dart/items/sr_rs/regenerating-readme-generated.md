Генератор преузима спецификацију са локално покренутог FastComments сервера
(`http://localhost:3001/js/swagger.json`) када је доступан, у супротном се враћа
на комитовани `openapi.json`.

```bash
python3 update.py
```

Захтева `node`/`npx` (за `@openapitools/openapi-generator-cli`) и Java.