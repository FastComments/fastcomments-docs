---  
לאחר קביעת תצורת SAML ב‑FastComments, עליך להגדיר את FastComments כספק שירות (Service Provider) בספק הזהות שלך.

### תצורת IdP כללית

רוב ספקי הזהות דורשים את המידע הבא כדי להוסיף את FastComments כיישום SAML:

#### מידע נדרש על ספק השירות

הערכים האלה נוצרים אוטומטית ומוצגים בדף תצורת SAML של FastComments:

**SP Entity ID / Audience**  
- פורמט: `https://fastcomments.com/saml/{your-tenant-id}`  
- מזהה באופן ייחודי את המופע של FastComments שלך  

**Assertion Consumer Service (ACS) URL**  
- פורמט: `https://fastcomments.com/saml/callback/{your-tenant-id}`  
- המקום שבו ה‑IdP שלך שולח תגובות SAML לאחר האימות  

**SP Metadata URL** *(אם נתמך על ידי ה‑IdP שלך)*  
- פורמט: `https://fastcomments.com/saml/metadata/{your-tenant-id}`  
- מספק תצורת SAML מלאה בפורמט XML  

**SAML Login URL**  
- פורמט: `https://fastcomments.com/saml/login/{your-tenant-id}`  
- קישור ישיר להתחלת אימות SAML  

### תכונות SAML נדרשות

הגדר את ספק הזהות שלך לשלוח את התכונות האלה עם תגובות SAML:

#### תכונות חיוניות

**כתובת דוא"ל** *(נדרש)*  
- **שם תכונה**: `email`, `emailAddress`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress`  
- **מטרה**: זיהוי משתמש ייחודי והודעות  
- **פורמט**: כתובת דוא"ל תקינה  

#### תכונות אופציונליות

**שם פרטי**  
- **שמות תכונות**: `firstName`, `givenName`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname`  
- **מטרה**: שם תצוגה של המשתמש  

**שם משפחה**  
- **שמות תכונות**: `lastName`, `surname`, or `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname`  
- **מטרה**: שם תצוגה של המשתמש  

**תפקידים** *(חשוב לבקרת גישה)*  
- **שמות תכונות**: `roles`, `groups`, `memberOf`, or custom attribute names  
- **מטרה**: הקצאת תפקידים והרשאות ב‑FastComments  
- **פורמט**: מערך של מחרוזות תפקיד או ערכים מופרדים בפסיקים  

### תצורות נפוצות של ספקי זהות

#### Microsoft Azure AD

1. **הוספת יישום ארגוני**  
   - חפש את "FastComments" או צור יישום SAML מותאם אישית  
   - השתמש במידע SP שסופק על ידי FastComments  

2. **הגדרת תכונות**  
   - דוא"ל: `user.mail` or `user.userprincipalname`  
   - שם פרטי: `user.givenname`  
   - שם משפחה: `user.surname`  
   - תפקידים: `user.assignedroles` or directory groups  

#### Okta

1. **יצירת יישום SAML**  
   - השתמש ב-"Create New App" ובחר SAML 2.0  
   - הגדר עם מידע SP של FastComments  

2. **הצהרות תכונה**  
   - דוא"ל: `user.email`  
   - שם פרטי: `user.firstName`  
   - שם משפחה: `user.lastName`  
   - תפקידים: `user.groups` or custom attributes  

#### Google Workspace

1. **הוספת יישום SAML**  
   - עבור ל‑Apps > Web and mobile apps > Add App > Add custom SAML app  
   - הגדר עם מידע SP של FastComments  

2. **מיפוי תכונות**  
   - דוא"ל: דוא"ל ראשי  
   - שם פרטי: שם פרטי  
   - שם משפחה: שם משפחה  
   - תפקידים: קבוצות או תכונות מותאמות  

#### Active Directory Federation Services (ADFS)

1. **הוספת אמון של צד תלוי**  
   - השתמש בכתובת URL של מטא‑נתונים של FastComments או בתצורה ידנית  
   - הגדר מידע SP כפי שסופק  

2. **כללי תביעות**  
   - דוא"ל: תביעת כתובת דוא"ל  
   - שם: תביעת Name ID  
   - תפקידים: חברות בקבוצה או תביעות מותאמות  

### גמישות בשם תכונה

FastComments מקבל מידע תפקידים ממספר שמות תכונות כדי להתאים לתצורות IdP שונות:

- `roles`  
- `groups`  
- `memberOf`  
- `role`  
- `group`  
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`  
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`  

גמישות זו מבטיחה תאימות עם ספקי זהות שונים מבלי לדרוש קונציות שמות תכונות ספציפיות.

### בדיקת התצורה שלך

לאחר תצורת ספק הזהות שלך:

1. שמור את תצורת ה‑IdP  
2. בדוק עם חשבון משתמש בדיקה ייעודי  
3. אמת שהתכונות נשלחות כראוי  
4. בדוק שהתפקידים ממופים כהלכה  
5. וודא שהזרם האימותי מסתיים בהצלחה  

רוב ספקי הזהות מציעים כלי בדיקת SAML לאימות התצורה לפני פריסת משתמשים בייצור.  
---