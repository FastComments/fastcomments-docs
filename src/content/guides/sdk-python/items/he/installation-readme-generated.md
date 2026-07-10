### התקנה מ‑GitHub

התקנה ישירה מתג שחרור (מומלץ, ניתן לשחזור מלא):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

קבעו את התג במקום סניף כדי שהבניות יהיו דטרמיניסטיות. אותה צורה עובדת ב‑`requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

לכל תוית [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) יש גלגל מצורף אם אתם מעדיפים להתקין ארטיפקט בינארי ישירות.

### תכולת הספרייה

ספרייה זו מכילה שני מודולים: לקוח ה‑API שנוצר והספרייה המרכזית של Python שמכילה כלי כתיבה ידנית שמקלים על העבודה עם ה‑API, כולל תמיכה ב‑SSO.

- [תיעוד ספריית לקוח ה‑API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [תיעוד הספרייה המרכזית, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API ציבוריים מול מאובטחים

ללקוח ה‑API יש שלוש מחלקות, `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות מפתח API, וה‑`PublicApi` מכילה שיטות שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/וכו׳ ללא אימות. ה‑`ModerationApi` מספקת סדרה נרחבת של API מודרציה בזמן אמת ומהירה. כל שיטה ב‑`ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או קובץ cookie של session ב‑FastComments.com.