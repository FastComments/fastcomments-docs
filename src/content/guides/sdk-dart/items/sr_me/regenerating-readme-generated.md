Generator uzima specifikaciju sa lokalno pokrenutog FastComments servera
(`http://localhost:3001/js/swagger.json`) kada je dostupna, u suprotnom se vraća
na commit‑ovanu `openapi.json`.

```bash
python3 update.py
```

Potrebni su `node`/`npx` (za `@openapitools/openapi-generator-cli`) i Java.