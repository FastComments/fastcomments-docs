### Using Nimble

```bash
nimble install fastcomments
```

### Building from Source

```bash
nimble build
```

### Library Contents

ספרייה זו מכילה את לקוח ה‑API שנוצר ואת כלי העזר של SSO כדי להקל על העבודה עם ה‑API.

- [תיעוד ספריית לקוח ה‑API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Public vs Secured APIs

ללקוח ה‑API קיימים שלושה מודולי API: `api_default`, `api_public`, ו‑`api_moderation`. המודול `api_default` מכיל שיטות הדורשות את מפתח ה‑API שלכם, וה‑`api_public` מכיל קריאות API שניתן לבצע ישירות מדפדפן/מכשיר נייד/וכו׳ ללא אימות. מודול `api_moderation` מכיל שיטות ללוח המחוונים של המפקח.

מודול `api_moderation` מספק חבילה נרחבת של API מודרציה חיים ומהירים. כל שיטת `api_moderation` מקבלת פרמטר `sso` וניתן לאמת באמצעות SSO או קובץ cookie של מושב FastComments.com.