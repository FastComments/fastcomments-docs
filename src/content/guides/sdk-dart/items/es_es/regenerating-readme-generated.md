El generador toma la especificación de un servidor FastComments en ejecución local (`http://localhost:3001/js/swagger.json`) cuando está disponible, de lo contrario recurre al `openapi.json` comprometido.

```bash
python3 update.py
```

Requiere `node`/`npx` (para `@openapitools/openapi-generator-cli`) y Java.