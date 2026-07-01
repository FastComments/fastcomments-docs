---
Generator pobiera specyfikację z lokalnie uruchomionego serwera FastComments
(`http://localhost:3001/js/swagger.json`) gdy jest dostępny, w przeciwnym razie przechodzi do
zatwierdzonego `openapi.json`.

```bash
python3 update.py
```

Wymaga `node`/`npx` (do `@openapitools/openapi-generator-cli`) oraz Java.
---