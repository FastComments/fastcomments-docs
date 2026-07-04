| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | מזהה השוכר של FastComments שלך (`demo` לבדיקה). |
| `API_KEY` | `""` | סוד ה-API שלך. חותם על SSO מאובטח ומאמת `admin()`. |
| `REGION` | `None` | `None` עבור ארה"ב, `"eu"` עבור אזור האיחוד האירופי. |
| `SSO.ENABLED` | `False` | הפעל את SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) או `"simple"` (לא חתום). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | מוצג למבקרים שלא מחוברים; ברירת המחדל היא `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | שדה FastComments למאפיין/נתיב/קריאה של משתמש. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` או נתיב מנוקד. |
| `SSO.USER_MAPPER` | `None` | נתיב מנוקד ל-`callable(user) -> dict`; העדיפות הגבוהה ביותר. |
| `WIDGET_DEFAULTS` | `{}` | תצורה הממוזגת לכל וידג'ט (מפתחי camelCase). |