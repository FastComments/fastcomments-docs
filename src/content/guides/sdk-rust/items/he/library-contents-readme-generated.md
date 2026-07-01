The FastComments Rust SDK consists of several modules:

- **Client Module** - לקוח API עבור FastComments REST APIs
  - הגדרות סוגים מלאות עבור כל מודלי ה-API
  - שלושה לקוחות API המכסים את כל השיטות של FastComments:
    - `default_api` (**DefaultApi**) - שיטות מאומתות במפתח API לשימוש בצד השרת
    - `public_api` (**PublicApi**) - שיטות ציבוריות, ללא מפתח API, בטוחות לשימוש מדפדפנים ואפליקציות מובייל
    - `moderation_api` (**ModerationApi**) - חבילה נרחבת של API של מודרציה בזמן אמת ומהירה. כל שיטת מודרציה מקבלת פרמטר `sso` ויכולה לאמת באמצעות SSO או עוגיית סשן של FastComments.com.
  - תמיכה מלאה ב-async/await עם tokio
  - ראה [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) לתיעוד API מפורט

- **SSO Module** - כלי Single Sign-On בצד השרת
  - יצירת אסימון מאובטח לאימות משתמש
  - תמיכה במצב SSO פשוט ומאובטח
  - חתימת אסימון מבוססת HMAC-SHA256

- **Core Types** - הגדרות סוג משותפות וכלים
  - מודלים של תגובות ומבני מטא-דאטה
  - הגדרות משתמש ודייר
  - פונקציות עזר לפעולות נפוצות