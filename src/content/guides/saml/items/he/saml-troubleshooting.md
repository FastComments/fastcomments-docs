מדריך זה מכסה בעיות נפוצות באימות SAML והפתרונות להן.

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- "Certificate validation failed" error
- משתמשים אינם מצליחים להשלים אימות SAML
- תגובות SAML נדחות

**Common Causes**:
- פורמט התעודה שגוי
- התעודה פגה
- סופקה תעודה שגויה
- תווים נוספים או רווחים בתעודה

**Solutions**:
1. **Verify Certificate Format**:
   - ודא שהתעודה כוללת את הסימונים `-----BEGIN CERTIFICATE-----` ו-`-----END CERTIFICATE-----`
   - הסר כל רווחים או שבירות שורה מיותרות
   - העתק את התעודה ישירות ממטאדטה או מהקונפיגורציה של ה-IdP

2. **Check Certificate Validity**:
   - ודא שהתעודה לא פגה
   - אשר שהתעודה מיועדת ל-IdP הנכון
   - השתמש בוולידטורים מקוונים של תעודות כדי לבדוק את הפורמט

3. **Re-download Certificate**:
   - הורד תעודה חדשה מה-IdP
   - השתמש ב-IdP metadata URL אם זמין
   - אשר שהתעודה תואמת לקונפיגורציה הנוכחית של ה-IdP

#### Signature Verification Failed

**Symptoms**:
- שגיאות אימות חתימה של אסרטיפיקציית SAML
- האימות נכשל לאחר כניסה ב-IdP
- הודעות שגיאה "Invalid signature"

**Solutions**:
1. **Algorithm Mismatch**:
   - בדוק שהאלגוריתם של החתימה ב-FastComments תואם את ה-IdP
   - נסה אלגוריתמים שונים לחתימה (SHA-256, SHA-1, SHA-512)
   - אמת שהאלגוריתם של הסיכום (digest) תואם לקונפיגורציה של ה-IdP

2. **Certificate Issues**:
   - ודא שהתעודה הנכונה לחתימה מוגדרת
   - אמת שהתעודה תואמת למפתח הפרטי שנמצא בשימוש ע"י ה-IdP
   - בדוק אם יש סיבוב תעודות (certificate rotation) ב-IdP

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- ה-IdP מדווח "Unknown Service Provider"
- תגובות SAML נשלחות ל-endpoint שגוי
- האימות לא מסתיים

**Solutions**:
1. **Verify SP Information**:
   - העתק במדויק את ה-Entity ID מהקונפיגורציה של FastComments
   - ודא ש-ACS URL תואם לפורמט: `https://fastcomments.com/saml/callback/{tenant-id}`
   - בדוק שגיאות הקלדה ב-tenant ID

2. **IdP Configuration**:
   - עדכן את ה-IdP עם ה-SP Entity ID הנכון
   - קבע את ה-ACS/Reply URL המתאים
   - אמת את הגדרות ה-binding של ה-IdP (HTTP-POST מועדף)

#### Missing or Incorrect Attributes

**Symptoms**:
- משתמשים נוצרים ללא תפקידים מתאימים
- חסר מידע בפרופיל המשתמש
- שגיאות "Email required"

**Solutions**:
1. **Email Attribute**:
   - ודא שה-IdP שולח את התכונה של המייל
   - בדוק מיפוי שמות התכונות (email, emailAddress, וכו')
   - אמת שהערך של המייל הוא כתובת מייל תקינה

2. **Role Attributes**:
   - אשר שה-IdP שולח מידע על תפקידים/קבוצות
   - בדוק ששמות תכונות התפקידים תואמים לציפיות של FastComments
   - אמת שערכי התפקיד תואמים בדיוק לשמות התפקידים ב-FastComments

3. **Attribute Format**:
   - בדוק גם פורמט מערך וגם פורמט מופרד בפסיקים עבור תפקידי משתמש
   - ודא שערכי התכונה אינם מכילים רווחים מיותרים
   - בדוק רגישות לאותיות בשמות תפקידים

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- הדפדפן מבצע הפניות אינסופיות בין FastComments ל-IdP
- האימות לעולם לא מסתיים
- מספר הפניות מוצג בכלי המפתחים של הדפדפן

**Solutions**:
1. **Check SP Configuration**:
   - ודא שה-Entity ID תואם בדיוק לקונפיגורציה של ה-IdP
   - ודא ש-ACS URL מוגדר נכון ב-IdP
   - בדוק סלאשים בסוף ה-URLs

2. **Session Issues**:
   - נקה עוגיות בדפדפן ונסה שוב
   - בדוק בחלון דפדפן פרטי/incognito
   - בדוק הגדרות timeout של סשן

#### Access Denied After Authentication

**Symptoms**:
- אימות SAML מצליח
- המשתמש מופרש חזרה ל-FastComments
- מוצגת הודעת "Access denied" או שגיאת הרשאות

**Solutions**:
1. **Role Assignment**:
   - ודא שלמשתמש יש תפקידים מתאימים ב-IdP
   - בדוק שהתכונת התפקיד נשלחת בתשובת ה-SAML
   - אשר ששמות התפקידים תואמים בדיוק לדרישות של FastComments

2. **Package Limitations**:
   - בדוק שלחשבון יש תוכנית Flex או Pro
   - ודא שתכונת ה-SAML מופעלת בחבילה
   - פנה לתמיכה אם החבילה כוללת SAML אך התכונה לא זמינה

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- הקצאות תפקידי אפליקציה לא משתקפות בטוקנים
- Claims לא נשלחות כראוי
- דרישות הקצאת משתמשים

**Solutions**:
- בדוק הקצאת משתמש לאפליקציית FastComments
- אמת שתפקידי האפליקציה מוגדרים כראוי
- ודא שמיפוי ה-claims כולל את התכונות הנדרשות

#### Okta

**Common Issues**:
- מסנני קבוצות לא עובדים כראוי
- הצהרות תכונה (attribute statements) מוגדרות לא נכון
- בעיות בהקצאת אפליקציה

**Solutions**:
- סקור את קונפיגורציית ה-attribute statements
- בדוק הקצאת קבוצות וכללי סינון
- אמת שהאפליקציה מוקצת למשתמשים/קבוצות המתאימות

#### Google Workspace

**Common Issues**:
- תכונות מותאמות אישית לא ממופות כראוי
- חברות בקבוצה לא נשלחות
- שגיאות בקונפיגורציית אפליקציית SAML

**Solutions**:
- הגדר סכימה מותאמת אישית לתכונות תפקידים
- בדוק הפצת חברות הקבוצה
- אמת מיפוי תכונות של אפליקציית SAML

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- תהליך האימות פג תוקף (timeout)
- "Request timeout" או שגיאות דומות
- זרימת אימות איטית

**Solutions**:
1. **Network Connectivity**:
   - בדוק שחוקי החומת אש מאפשרים תקשורת עם FastComments
   - אמת רזולוציית DNS עבור fastcomments.com
   - בדוק חיבור רשת מה-IdP אל FastComments

2. **Performance Issues**:
   - בדוק זמני תגובה של ה-IdP
   - אמת שלאבדיקת שרשרת התעודות אין קושי שגורם לעיכוב
   - שקול השפעת השיהוי ברשת בין ה-IdP והמשתמשים

#### SSL/TLS Issues

**Symptoms**:
- אזהרות תעודה במהלך האימות
- כשלי SSL handshake
- שגיאות "Secure connection failed"

**Solutions**:
- ודא שכל נקודות הקצה של SAML משתמשות ב-HTTPS
- בדוק תוקף תעודות לכל הדומיינים המעורבים
- אמת תאימות גרסת TLS

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - עקוב אחרי לשונית ה-Network במהלך זרימת SAML
   - בדוק את ה-Console לשגיאות JavaScript
   - בחן בקשות POST של SAML (אם נראות)

2. **IdP Logging**:
   - אפשר דיבוג של SAML ב-IdP שלך
   - סקור לוגים של ה-IdP לפרטי בקשות/תגובות SAML
   - בדוק בעיות במיפוי תכונות

#### Common Log Messages

**FastComments Logs**:
- "SAML config not found" - SAML לא מופעל או מוגדר לא נכון
- "Invalid certificate" - אימות התעודה נכשל
- "Missing email attribute" - המייל הנדרש לא נשלח בתשובת ה-SAML

**IdP Logs**:
- "Unknown service provider" - חוסר התאמה של Entity ID
- "Invalid ACS URL" - כתובת Assertion Consumer Service שגויה
- "User not assigned" - למשתמש אין גישה לאפליקציית SAML

### Getting Help

#### Information to Gather

כאשר פונים לתמיכה, ספק:
- הודעות שגיאה מדויקות וחותמות זמן
- פרטי קונפיגורציית SAML (ללא מידע רגיש)
- סוג וגרסת ה-IdP
- צעדים לשחזור הבעיה
- מידע על הדפדפן והרשת

#### FastComments Support

For SAML-related issues:
1. Use the [פורטל התמיכה](https://fastcomments.com/auth/my-account/help)
2. כלול את tenant ID ודוא"לי המשתמשים המושפעים
3. ספק הודעות שגיאה ופרטי קונפיגורציה
4. ציין את סוג ה-IdP ושיטת הקונפיגורציה

#### IdP Support

For IdP-specific issues:
- עיין בתיעוד ה-IdP לפתרון בעיות SAML
- השתמש בערוצי התמיכה של ה-IdP לבעיות קונפיגורציה
- נצל פורומים קהילתיים של ה-IdP לבעיות נפוצות

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - בדוק שינויים בקונפיגורציה בסביבה שאינה פרודקשן
   - אמת עם מספר משתמשי בדיקה
   - תעד קונפיגורציות שפועלות

2. **Monitor Regularly**:
   - הגדר ניטור לכשלי אימות SAML
   - סקור תאריכי פקיעת תעודות
   - נטר שינויים בקונפיגורציית ה-IdP

3. **Documentation**:
   - תחזק תיעוד של קונפיגורציית SAML
   - תעד כל קונפיגורציה מותאמת אישית או פתרונות עקיפה
   - שמור פרטי קשר למנהלי ה-IdP

#### Proactive Maintenance

1. **Certificate Management**:
   - נטר תאריכי פקיעת תעודות
   - תכנן נוהלי סיבוב תעודות (certificate rotation)
   - בדוק עדכוני תעודות לפני פקיעת התוקף

2. **Configuration Reviews**:
   - סקור באופן קבוע את קונפיגורציית SAML
   - אמת שהגדרות ה-IdP נשארות עדכניות
   - עדכן תיעוד כאשר מבוצעים שינויים