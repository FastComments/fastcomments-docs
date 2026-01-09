הגדרת אימות SAML ב‑FastComments דורשת הן תצורה בלוח הניהול שלך והן הגדרות אצל ספק הזהות שלך.

### דרישות מוקדמות

לפני תצורת SAML, ודא שיש לך:

- תוכנית FastComments Flex או Pro (SAML אינו זמין בתוכנית Creators)
- גישה מנהלית לחשבון ה‑FastComments שלך
- גישה מנהלית אצל ספק הזהות שלך
- מטא‑נתוני SAML של ה‑IdP שלך או מידע על התעודה שלו

### גישה לתצורת SAML

1. היכנס/י ל[לוח הניהול של FastComments](https://fastcomments.com/auth/my-account)
2. עבור אל **API/SSO Settings** בסרגל הצד השמאלי
3. לחץ/י על כפתור **SAML Config**

אם אינך רואה את כפתור ה‑SAML Config, אמת כי:
- לחשבונך יש את החבילה הנדרשת (Flex או Pro)
- יש לך הרשאות מנהל
- למשתמש שלך יש תפקידי API Admin או Admin Admin

### תצורת SAML בסיסית

#### הפעלת אימות SAML

1. סמן/י את התיבה **Enable SAML Authentication**
2. פעולה זו מפעילה את SAML עבור השוכר שלך ומציגה את שדות התצורה

#### שדות חובה

**IdP Single Sign-On URL** *(Required)*
- ה‑URL שאליו המשתמשים יופנו לאימות
- בדרך כלל מסופק על‑ידי ספק הזהות שלך
- דוגמה: `https://your-company.okta.com/app/fastcomments/sso/saml`

**IdP X.509 Certificate** *(Required)*
- התעודה הציבורית מספק הזהות שלך
- משמשת לאימות האותנטיות של תגובות SAML
- חייבת לכלול את התעודה המלאה עם סימני BEGIN/END
- פורמט לדוגמה:
```
-----BEGIN CERTIFICATE-----
MIICXjCCAcegAwIBAgIBADANBgkqhkiG9w0BAQsFADA...
-----END CERTIFICATE-----
```

#### שדות אופציונליים

**IdP Entity ID / Issuer**
- מזהה את ספק הזהות שלך
- אם נשאר ריק, ברירת המחדל היא ה‑URL של FastComments שלך
- צריך להתאים ל‑issuer המוגדר ב‑IdP שלך

### תצורה מתקדמת

#### הגדרות אבטחה

**Signature Algorithm**
- ברירת מחדל SHA-256 (מומלץ)
- אפשרויות: SHA-1, SHA-256, SHA-512
- צריך להתאים להגדרות ה‑IdP שלך

**Digest Algorithm**
- ברירת מחדל SHA-256 (מומלץ)
- משמש לחישוב digest בתגובות SAML
- צריך להתאים להגדרות ה‑IdP שלך

**Name ID Format**
- ברירת מחדל: פורמט כתובת אימייל
- קובע כיצד מזהי משתמשים מעוצבים
- אפשרויות נפוצות: Email Address, Persistent, Transient

#### הצפנה (אופציונלי)

**Private Key for Decryption**
- נדרש רק אם ה‑IdP שלך מוצפן את ההצהרות (assertions) של SAML
- הדבק/י כאן את המפתח הפרטי המשמש לפענוח
- ברוב הפריסות אין צורך בהצפנת ה‑assertion

### שמירת התצורה

1. סקור/י את כל ההגדרות לוודא שהן נכונות
2. לחץ/י על **Save SAML Configuration**
3. המערכת תבצע אימות של התצורה שלך
4. אם האימות מצליח, תראה/י הודעת אישור

### שלבים הבאים

לאחר שמירת תצורת SAML של FastComments שלך:

1. הגדר/י את ספק הזהות שלך באמצעות המידע של ה‑Service Provider
2. בדוק/י את מהלך האימות
3. קבע/י תפקידי משתמש והרשאות לפי הצורך

מידע ה‑Service Provider הדרוש להגדרת ה‑IdP שלך יוצג ברגע ש‑SAML יופעל.