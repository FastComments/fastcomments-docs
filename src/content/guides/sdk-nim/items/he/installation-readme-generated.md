### שימוש ב-Nimble

```bash
nimble install fastcomments
```

### בנייה מקוד המקור

```bash
nimble build
```

### תוכן הספרייה

ספרייה זו מכילה את לקוח ה-API שנוצר באופן אוטומטי ואת כלי ה-SSO שמקלים על העבודה עם ה-API.

- [תיעוד ספריית לקוח ה-API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### ממשקי API ציבוריים מול מאובטחים

עבור לקוח ה-API קיימים שלושה מודולי API: `api_default`, `api_public` ו-`api_moderation`. המודול `api_default` מכיל שיטות הדורשות את מפתח ה-API שלכם, ואילו `api_public` מכיל קריאות API
שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו' ללא אימות. המודול `api_moderation` מכיל שיטות עבור לוח המחוונים של המנחה.

המודול `api_moderation` מספק חבילה נרחבת של ממשקי API לניהול תגובות מהיר ובזמן אמת. כל שיטה ב-`api_moderation` מקבלת פרמטר `sso` ויכולה לבצע אימות באמצעות SSO או באמצעות עוגיית סשן של FastComments.com.