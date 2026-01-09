FastComments ממפה תפקידי משתמש של SAML להרשאות פנימיות, ומאפשר בקרת גישה מבוססת-תפקידים עבור הארגון שלך.

### מערכת התפקידים של FastComments

FastComments משתמשת במערכת הרשאות מבוססת תפקידים שבה למשתמשים יכולים להיות תפקידים אחדים או יותר שקובעים את רמות הגישה והיכולות שלהם.

### תפקידי FastComments הזמינים

#### תפקידים ניהוליים

**fc-account-owner**
- **Permissions**: גישה ניהולית מלאה
- **Capabilities**: כל התכונות, ניהול תשלומים, ניהול משתמשים
- **Use Case**: מנהלי חשבון ראשיים ובעלי חשבון

**fc-admin-admin**  
- **Permissions**: גישה ניהולית לרוב התכונות
- **Capabilities**: ניהול משתמשים, קונפיגורציה, מתן חסימות/בקרה. **יכול לנהל מנהלים אחרים.**
- **Use Case**: מנהלים משניים וצוותי IT

**fc-billing-admin**
- **Permissions**: ניהול חשבוניות ומנויים
- **Capabilities**: שיטות תשלום, חשבוניות, שינויים במנוי
- **Use Case**: חברי צוות הכספים ונציגי חיוב

#### תפקידים מיוחדים

**fc-analytics-admin**
- **Permissions**: גישה לניתוחים ודיווח
- **Capabilities**: צפייה בסטטיסטיקות האתר, נתוני מעורבות משתמשים
- **Use Case**: צוותי שיווק ואנליסטים נתונים

**fc-api-admin**
- **Permissions**: גישה וניהול API
- **Capabilities**: אישורי API, קונפיגורציית webhooks
- **Use Case**: מפתחים ומשלבים טכניים

**fc-moderator**
- **Permissions**: יכולות מתווך תגובות
- **Capabilities**: אישור/דחייה של תגובות, ניהול ספאם
- **Use Case**: מפקחי קהילה ומנהלי תוכן

### תצורת מיפוי תפקידים

#### מקורות מאפייני SAML

FastComments מקבלת מידע על תפקידים משמות מאפייני SAML שונים כדי להבטיח תאימות עם ספקי זהות שונים:

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### פורמטים נתמכים של תפקידים

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### קונפיגורציית תפקידי ספק זהות

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - הגדר את תפקידי FastComments באפליקציית Azure AD שלך
   - הקצה משתמשים לתפקידי היישום המתאימים
   - קנפג claims לכלול את התפקידים שהוקצו

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - צור קבוצות התואמות לשמות תפקידי FastComments
   - הקצה משתמשים לקבוצות המתאימות
   - קנפג הצהרות מאפיינים

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - צור יחידות ארגוניות או קבוצות
   - תן שמות לקבוצות עם קידומות תפקידי FastComments
   - קנפג מיפוי מאפיינים

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### התנהגות ברירת מחדל של משתמשים

#### משתמשים ללא תפקידים

כאשר למשתמש SAML אין תפקידים או שיש לו תפקידים שלא מזוהים:
- המשתמש נוצר כמתגיב סטנדרטי
- לא ניתנת גישה ניהולית
- יכול לפרסם ולנהל את התגובות שלו בלבד
- לא יכול לגשת לפונקציות לוח הניהול

#### ירושת תפקידים

- למשתמשים יכולים להיות מספר תפקידים בו-זמנית
- ההרשאות מצטברות (חל הרמה הגבוהה ביותר של הרשאה)
- שינויים בתפקידים ב-IdP משתקפים בכניסה הבאה

### ניהול משתמשי SAML

#### יצירת משתמש

כאשר משתמש נכנס דרך SAML בפעם הראשונה:
1. **User Account**: נוצר אוטומטית כשהמייל משמש כמזהה
2. **Role Assignment**: התפקידים מוחלים בהתבסס על מאפייני SAML
3. **Profile Information**: שם פרטי/משפחה מתמלאים אם סופקו
4. **Permission Activation**: התפקידים הופכים פעילים מיד

#### עדכוני תפקידים

משתמשי SAML קיימים מקבלים עדכוני תפקידים:
1. **Login Trigger**: עדכוני תפקידים מתבצעים במהלך כל כניסת SAML
2. **Immediate Effect**: הרשאות חדשות חלות מיד
3. **Role Removal**: תפקידים שהוסרו מבוטלים אוטומטית
4. **Audit Trail**: שינויים בתפקידים מתועדיים ביומני ביקורת

### מיפוי תפקידים מותאם

#### התאמה ארגונית

ללקוחות ארגוניים עם דרישות מיוחדות:
- ניתן למפות שמות תפקידים מותאמים להרשאות FastComments
- ניתן ליישם היררכיות תפקידים מורכבות
- ניתן להגדיר בקרות גישה ספציפיות למחלקות

פנה לתמיכת FastComments עבור תצורות מיפוי תפקידים מותאמות.

#### אימות תפקידים

FastComments מאמתת תפקידים נכנסים:
- תפקידים שלא מזוהים מתעלמים (לא נדחים)
- מאפייני תפקידים פגומים מתועדים לצורכי פתרון בעיות
- משתמשים שומרים על התפקידים הקיימים אם היסטית SAML חסרת מידע על תפקידים

### שיטות מומלצות

#### ניהול תפקידים

1. **Principle of Least Privilege**: הקצה את ההרשאות המינימליות הנחוצות
2. **Regular Auditing**: בדוק תפקידים וגישה באופן תקופתי  
3. **Clear Naming**: השתמש בשמות קבוצות תיאוריים ב-IdP שלך
4. **Documentation**: תחזק תיעוד של הקצאות תפקידים

#### שיקולי אבטחה

1. **Role Attributes**: ודא שמאפייני התפקיד מאובטחים כראוי בתגובות SAML
2. **Attribute Validation**: אמת שרק מערכות מורשות יכולות להקצות תפקידים
3. **Access Reviews**: בדוק באופן תדיר הקצאות תפקידים ניהוליים
4. **Monitoring**: נטר שינויים בתפקידים ופעולות ניהוליות

### פתרון בעיות בתפקידים

#### בעיות נפוצות

**Roles Not Applied**:
- בדוק ששמות מאפייני SAML תואמים לפורמטים הנתמכים
- אמת שספק הזהות שולח מידע תפקידים
- אשר שערכי התפקיד תואמים בדיוק לשמות תפקידי FastComments

**Access Denied**:
- ודא שלמשתמש מוקצה תפקיד מתאים ב-IdP
- בדוק איות ורגישות לאותיות גדולות/קטנות של התפקיד
- אשר שהתפקיד מעוצב כראוי בתשובת SAML

**Missing Permissions**:
- בדוק את הגדרות התפקידים וההרשאות הנדרשות
- בדוק הקצאות תפקידים מתנגשות
- אמת שהמשתמש נכנס לאחר שינויים בתפקידים