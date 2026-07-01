Generator preuzima specifikaciju s lokalno pokrenutog FastComments poslužitelja (`http://localhost:3001/js/swagger.json`) kada je dostupna, a u suprotnom se vraća na `openapi.json` iz commita.

```bash
python3 update.py
```

Zahtijeva `node`/`npx` (za `@openapitools/openapi-generator-cli`) i Java.