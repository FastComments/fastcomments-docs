Ο γεννήτρια διαβάζει το spec από έναν τοπικά εκτελούμενο διακομιστή FastComments (`http://localhost:3001/js/swagger.json`) όταν είναι διαθέσιμο, διαφορετικά επανέρχεται στο δεσμευμένο `openapi.json`.

```bash
python3 update.py
```

Απαιτείται `node`/`npx` (για `@openapitools/openapi-generator-cli`) και Java.