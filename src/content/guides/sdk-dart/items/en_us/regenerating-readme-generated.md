The generator picks up the spec from a locally running FastComments server
(`http://localhost:3001/js/swagger.json`) when available, otherwise falls back
to the committed `openapi.json`.

```bash
python3 update.py
```

Requires `node`/`npx` (for `@openapitools/openapi-generator-cli`) and Java.