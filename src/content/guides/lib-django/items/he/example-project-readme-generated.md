---
תצוגה ניתנת להרצה נמצאת ב-[`example/`](https://github.com/FastComments/fastcomments-django/tree/main/example): אפליקציית מסלול שמאל + שלב ראשי עם דף לכל וידג'ט ו**דף כניסה שמציג משתמשי הדגמה שהוזנו מראש**.  
היכנס כאחד מהם והווידג'טים של תגובות וצ'אט חי מאמתים את הזהות דרך **Secure SSO**. מתיקייה זו:

```bash
python manage.py migrate
# Use your own tenant to see Secure SSO in action (an API secret enables it):
FASTCOMMENTS_TENANT_ID=... FASTCOMMENTS_API_KEY=... python manage.py runserver
```

בלי סוד API הוא חוזר לשוכר הציבורי `demo` (אנונימי).  
[`example/browser_smoke.py`](https://github.com/FastComments/fastcomments-django/blob/main/example/browser_smoke.py) הוא מבחן קצה-לקצה (e2e) של Playwright שטוען את הדף בדפדפן אמיתי ומפרסם תגובה כמשתמש Secure-SSO.  
---