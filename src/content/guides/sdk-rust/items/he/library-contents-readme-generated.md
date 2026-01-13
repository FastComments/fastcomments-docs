---
ערכת ה-SDK של FastComments עבור Rust מורכבת ממספר מודולים:

- **Client Module** - לקוח API שנוצר אוטומטית עבור ממשקי ה-REST של FastComments
  - הגדרות טיפוסים מלאות לכל דגמי ה-API
  - נקודות קצה מאומתות (`DefaultApi`) וציבוריות (`PublicApi`)
  - תמיכה מלאה ב-async/await עם tokio
  - ראה [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) לתיעוד מפורט של ה-API

- **SSO Module** - כלי SSO בצד השרת
  - יצירת אסימונים מאובטחת לאימות משתמשים
  - תמיכה הן במצב SSO פשוט והן במצב SSO מאובטח
  - חתימת אסימונים מבוססת HMAC-SHA256

- **Core Types** - הגדרות סוגים וכלי עזר משותפים
  - מודלים של תגובות ומבני מטא-נתונים
  - הגדרות משתמש ושוכר
  - פונקציות עזר לפעולות נפוצות
---