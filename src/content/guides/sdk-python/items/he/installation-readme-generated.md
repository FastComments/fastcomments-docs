### התקנה מ‑GitHub

התקנה ישירה מתג שחרור (מומלץ, ניתן לשחזור מלא):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

קבעו את התג במקום סניף כדי שהבניות יהיו דטרמיניסטיות. הצורה הזו עובדת גם ב‑`requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

כל תוית של [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) כולל גם גלגל (wheel) שנבנה מצורף אם אתם מעדיפים להתקין ארטיפקט בינארי ישירות.

### תוכן הספרייה

ספרייה זו מכילה שני מודולים: לקוח ה‑API שנוצר והספרייה המרכזית של Python שמכילה כלים שנכתבו ידנית כדי להקל על העבודה עם ה‑API, כולל תמיכה ב‑SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API ציבוריים מול מאובטחים

ללקוח ה‑API יש שלוש מחלקות: `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות מפתח API שלכם, וה‑`PublicApi` מכילה שיטות שניתן לקרוא להן ישירות מדפדפן/מכשיר נייד/וכו׳ ללא אימות. ה‑`ModerationApi` מספקת סדרה רחבה של API מודרציה בזמן אמת ומהירה. כל שיטה ב‑`ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או קובץ cookie של session ב‑FastComments.com.