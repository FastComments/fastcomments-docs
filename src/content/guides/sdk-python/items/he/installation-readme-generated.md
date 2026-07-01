### התקנה מ‑GitHub

התקן ישירות מתג רילייז (מומלץ, ניתן לשחזר במלואו):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

נעץ את התג במקום סניף כך שהבנייה תהיה דטרמיניסטית. אותה הצורה עובדת ב‑`requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

לכל תיוג [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) יש גלגל (wheel) שנבנה מצורף אם אתה מעדיף להתקין ארטיפקט בינארי ישירות.

### תוכן הספרייה

ספרייה זו מכילה שני מודולים: לקוח ה‑API שנוצר והספרייה המרכזית של פייתון שמכילה כלי כתובים ידנית כדי להקל על העבודה עם ה‑API, כולל תמיכה ב‑SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### API ציבוריים לעומת מאובטחים

ללקוח ה‑API יש שלושה מחלקות, `DefaultApi`, `PublicApi`, ו‑`ModerationApi`. ה‑`DefaultApi` מכילה שיטות הדורשות את מפתח ה‑API שלך, ו‑`PublicApi` מכילה שיטות שניתן לבצע ישירות מדפדפן/התקן נייד/וכו' ללא אימות. ה‑`ModerationApi` מספקת חבילה נרחבת של API למודרציה חיה ומהירה. כל שיטת `ModerationApi` מקבלת פרמטר `sso` ויכולה לבצע אימות דרך SSO או באמצעות עוגיית סשן של FastComments.com.