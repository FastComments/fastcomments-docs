---
Генераторът извлича спецификацията от локално работещ FastComments сървър
(`http://localhost:3001/js/swagger.json`), когато е наличен, в противен случай
се връща към комитнатия `openapi.json`.

```bash
python3 update.py
```

Изисква `node`/`npx` (за `@openapitools/openapi-generator-cli`) и Java.
---