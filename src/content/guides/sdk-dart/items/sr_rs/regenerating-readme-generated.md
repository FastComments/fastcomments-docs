Generator preuzima specifikaciju sa lokalno pokrenutog FastComments servera
(`http://localhost:3001/js/swagger.json`) kada je dostupna, inače se vraća na
komitovani `openapi.json`.

```bash
python3 update.py
```

Zahteva `node`/`npx` (za `@openapitools/openapi-generator-cli`) i Java.