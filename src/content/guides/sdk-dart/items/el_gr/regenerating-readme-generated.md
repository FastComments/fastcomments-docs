---
Ο δημιουργός ανακτά το spec από έναν τοπικά εκτελούμενο διακομιστή FastComments (`http://localhost:3001/js/swagger.json`) όταν είναι διαθέσιμο, διαφορετικά επιστρέφει στο δεσμευμένο `openapi.json`.

```bash
python3 update.py
```

Απαιτεί `node`/`npx` (για `@openapitools/openapi-generator-cli`) και Java.
---