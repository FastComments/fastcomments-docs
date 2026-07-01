---
Der Generator nimmt die Spezifikation von einem lokal laufenden FastComments‑Server
(`http://localhost:3001/js/swagger.json`) wenn verfügbar, sonst fällt er zurück
zur festgeschriebenen `openapi.json`.

```bash
python3 update.py
```

Benötigt `node`/`npx` (für `@openapitools/openapi-generator-cli`) und Java.
---