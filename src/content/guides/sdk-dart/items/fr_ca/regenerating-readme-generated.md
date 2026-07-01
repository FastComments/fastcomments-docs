---
Le générateur récupère la spécification à partir d'un serveur FastComments en cours d'exécution localement (`http://localhost:3001/js/swagger.json`) lorsqu'elle est disponible, sinon il revient au fichier `openapi.json` engagé.

```bash
python3 update.py
```

Nécessite `node`/`npx` (pour `@openapitools/openapi-generator-cli`) et Java.
---