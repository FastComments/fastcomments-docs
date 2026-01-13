### הרצת בדיקות

```bash
# הגדר משתני סביבה
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# הפעל בדיקות
pytest tests/
```

### יצירת הלקוח מחדש

כדי ליצור את לקוח ה-API מחדש מהמפרט העדכני של OpenAPI:

```bash
./update.sh
```

זה יבצע:
1. הורדת המפרט העדכני של OpenAPI משרת FastComments פעיל (או שימוש בקובץ מקומי openapi.yaml)
2. יצירת קוד לקוח ה-Python
3. השטחת מבנה התיקיות
4. ניקוי קבצי תצורה מיותרים