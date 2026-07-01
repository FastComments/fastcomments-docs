Il generatore preleva la specifica da un server FastComments in esecuzione localmente (`http://localhost:3001/js/swagger.json`) quando disponibile, altrimenti ripiega sul `openapi.json` committato.

```bash
python3 update.py
```

Richiede `node`/`npx` (per `@openapitools/openapi-generator-cli`) e Java.