---
Генератор отримує специфікацію з локально запущеного сервера FastComments (`http://localhost:3001/js/swagger.json`), якщо вона доступна; в іншому випадку використовується комітований `openapi.json`.

```bash
python3 update.py
```

Потрібні `node`/`npx` (для `@openapitools/openapi-generator-cli`) та Java.
---