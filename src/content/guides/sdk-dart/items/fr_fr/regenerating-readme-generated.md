Le générateur récupère la spécification depuis un serveur FastComments en cours d'exécution localement (`http://localhost:3001/js/swagger.json`) lorsqu'il est disponible, sinon il revient au fichier `openapi.json` engagé.

```bash
python3 update.py
```

Nécessite `node`/`npx` (pour `@openapitools/openapi-generator-cli`) et Java.