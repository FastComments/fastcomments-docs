| מפתח | משתנה סביבה | ברירת מחדל | תיאור |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | מזהה השוכר (tenant) שלך ב-FastComments |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | מפתח API לקריאות בצד השרת |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (ארה"ב) או `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | הפעל SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` או `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | כתובת URL לכניסה (תחזור לנתיב של Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | כתובת URL להתנתקות (תחזור לנתיב של Laravel) |
| `widget_defaults` | — | `[]` | הגדרות ברירת מחדל לווידג'ט |