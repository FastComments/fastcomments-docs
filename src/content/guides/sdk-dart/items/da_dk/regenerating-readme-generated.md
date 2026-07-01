Generatoren henter specifikationen fra en lokalt kørende FastComments‑server (`http://localhost:3001/js/swagger.json`), når den er tilgængelig, ellers falder den tilbage til den committede `openapi.json`.

```bash
python3 update.py
```

Kræver `node`/`npx` (for `@openapitools/openapi-generator-cli`) og Java.