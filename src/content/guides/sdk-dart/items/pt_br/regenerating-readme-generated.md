---
O gerador obtém a especificação de um servidor FastComments em execução local
(`http://localhost:3001/js/swagger.json`) quando disponível, caso contrário recua
para o `openapi.json` comprometido.

```bash
python3 update.py
```

Requer `node`/`npx` (para `@openapitools/openapi-generator-cli`) e Java.
---