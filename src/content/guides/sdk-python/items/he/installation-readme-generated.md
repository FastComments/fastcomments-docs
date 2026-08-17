### התקנה מ‑GitHub

התקן ישירות מתג שחרור (מומלץ, ניתן לשחזור מלא):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

קבע את התג במקום סניף כדי שהבניות יהיו דטרמיניסטיות. הצורה הזו עובדת גם ב‑`requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

כל תוית [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) כוללת גם גלגל (wheel) שנבנה מצורף אם אתה מעדיף להתקין ארטיפקט בינארי ישירות.

### תוכן הספרייה

ספרייה זו מכילה שני מודולים: לקוח ה‑API שנוצר והספרייה המרכזית של Python שמכילה כלים שנכתבו ידנית כדי להקל על העבודה עם ה‑API, כולל תמיכה ב‑SSO.

- [תיעוד ספריית לקוח ה‑API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [תיעוד הספרייה המרכזית, כולל דוגמאות SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API ציבוריים מול מאובטחים

עבור לקוח ה‑API, קיימות שלוש מחלקות, `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות את מפתח ה‑API שלך, וה‑`PublicApi` מכילה שיטות שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות. ה‑`ModerationApi` מספקת סדרה נרחבת של API מודרציה בזמן אמת ומהירה. כל שיטה של `ModerationApi` מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או קוביית סשן של FastComments.com.