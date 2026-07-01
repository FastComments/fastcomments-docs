---
Generator preuzima specifikaciju sa lokalno pokrenutog FastComments servera
(`http://localhost:3001/js/swagger.json`) kada je dostupan, u suprotnom se vraća
na commitovani `openapi.json`.

```bash
python3 update.py
```

Potrebni su `node`/`npx` (za `@openapitools/openapi-generator-cli`) i Java.
---