ערכת ה-SDK של FastComments עבור Swift מורכבת ממספר מודולים:

- **מודול לקוח** - לקוח API שנוצר אוטומטית עבור ממשקי ה-REST של FastComments
  - הגדרות טיפוס מלאות לכל דגמי ה-API
  - נקודות קצה מאובטחות (`DefaultAPI`) וציבוריות (`PublicAPI`)
  - תמיכה מלאה ב-async/await
  - עיין ב-[client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) לתיעוד מפורט של ה-API

- **מודול SSO** - כלי עזר ל-Single Sign-On בצד השרת
  - יצירת אסימונים מאובטחים לאימות משתמשים
  - תמיכה במצבי SSO פשוטים ומאובטחים
  - חתימת אסימונים מבוססת HMAC-SHA256 באמצעות CryptoKit