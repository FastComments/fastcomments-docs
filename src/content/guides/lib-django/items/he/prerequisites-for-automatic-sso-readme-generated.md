---
כדי להעביר את המשתמש המחובר לוידג׳ט באופן אוטומטי, התגים קוראים את המשתמש הנוכחי מהבקשה. ודא שהפרויקט שלך כולל את שני הדברים האלה (הם מופעלים כברירת מחדל בפרויקט Django סטנדרטי):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

בלי בקשה בקונטקסט של התבנית, הווידג׳טים מוצגים למבקר אנונימי. תמיד אפשר להעביר משתמש במפורש: `{% fastcomments user=some_user %}`.
---