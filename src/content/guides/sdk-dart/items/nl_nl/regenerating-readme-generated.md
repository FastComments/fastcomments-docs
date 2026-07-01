---
De generator haalt de specificatie op van een lokaal draaiende FastComments‑server (`http://localhost:3001/js/swagger.json`) wanneer beschikbaar, anders valt deze terug op het gecommitte `openapi.json`.

```bash
python3 update.py
```

Vereist `node`/`npx` (voor `@openapitools/openapi-generator-cli`) en Java.
---