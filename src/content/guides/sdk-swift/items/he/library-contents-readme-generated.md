ה‑FastComments Swift SDK מורכב ממספר מודולים:

- **Client Module** - לקוח API עבור REST APIs של FastComments
  - הגדרות טיפוסים מלאות לכל מודלי ה‑API
  - שיטות מאומתות (`DefaultAPI`), ציבוריות (`PublicAPI`), ושיטות למודרציה (`ModerationAPI`)
  - תמיכה מלאה ב‑async/await
  - ראה [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) לתיעוד מפורט של ה‑API

- **SSO Module** - כלים ל‑Single Sign‑On בצד השרת
  - יצירת אסימונים מאובטחת לאימות משתמשים
  - תמיכה בשני מצבי SSO — פשוט ומאובטח
  - חתימת אסימונים מבוססת HMAC-SHA256 באמצעות CryptoKit