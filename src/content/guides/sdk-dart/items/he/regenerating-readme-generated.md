הגנרטור קולט את המפרט משרת FastComments שמופעל מקומית (`http://localhost:3001/js/swagger.json`) כאשר הוא זמין, אחרת הוא נופל חזרה ל‑`openapi.json` המחוייב.

```bash
python3 update.py
```

דורש `node`/`npx` (ל‑`@openapitools/openapi-generator-cli`) ו‑Java.