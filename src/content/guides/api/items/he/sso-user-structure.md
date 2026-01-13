FastComments מספק פתרון SSO קל לשימוש. עדכון מידע משתמש עם אינטגרציה מבוססת HMAC הוא
פשוט כמו לגרום למשתמש לטעון את העמוד עם מטען מעודכן.

עם זאת, ייתכן שיהיה רצוי לנהל משתמש מחוץ לזרימה זו, כדי לשפר את עקביות האפליקציה שלך.

ה-API של משתמש SSO מספק דרך לבצע פעולות CRUD על אובייקטים שאנחנו קוראים להם SSOUsers. אובייקטים אלה שונים ממשתמשים רגילים ו
נשמרים בנפרד לבטיחות טיפוסים.

המבנה עבור אובייקט SSOUser הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. **/
        badgeIds: string[]
        /** If true, replaces all existing displayed badges with the provided ones. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### חיוב עבור משתמשי SSO

משתמשי SSO מחויבים בצורה שונה בהתבסס על דגלי ההרשאות שלהם:

- **משתמשי SSO רגילים**: משתמשים ללא הרשאות מנהל או מנהל תוכן מחויבים כמשתמשי SSO רגילים
- **מנהלי SSO**: משתמשים עם דגלי `isAccountOwner` או `isAdminAdmin` מחויבים בנפרד כמנהלי SSO (אותו תעריף כמנהלי שוכר רגילים)
- **מנהלי תוכן SSO**: משתמשים עם דגל `isCommentModeratorAdmin` מחויבים בנפרד כמנהלי תוכן SSO (אותו תעריף כמנהלי תוכן רגילים)

**חשוב**: כדי למנוע חיוב כפול, המערכת מסירה אוטומטית כפילויות של משתמשי SSO מול משתמשי שוכר רגילים ומנהלי תוכן לפי כתובת אימייל. אם למשתמש SSO יש את אותו אימייל כמו משתמש שוכר רגיל או מנהל תוכן, הוא לא יחויב פעמיים.

### בקרת גישה

ניתן לחלק משתמשים לקבוצות. לשם כך משמש שדה `groupIds`, והוא אופציונלי.

### @אזכורים

כברירת מחדל `@mentions` ישתמש ב-`username` כדי לחפש משתמשי sso אחרים כאשר מקלידים את התו `@`. אם משתמשים ב-`displayName`, אז תוצאות התואמות
ל-`username` יתעלמו כשיש התאמה ל-`displayName`, ותוצאות החיפוש של `@mention` ישתמשו ב-`displayName`.

### מנויים

עם FastComments, משתמשים יכולים להירשם לעמוד על ידי לחיצה על סמל הפעמון בווידג'ט התגובות ולחיצה על הירשם.

עם משתמש רגיל, אנחנו שולחים להם אימיילי התראות על בסיס הגדרות ההתראות שלהם.

עם משתמשי SSO, אנחנו מפצלים זאת לתאימות לאחור. משתמשים יקבלו את אימיילי התראות המנוי הנוספים האלה
רק אם תגדיר `optedInSubscriptionNotifications` ל-`true`.

### תגים

אתה יכול להקצות תגים למשתמשי SSO באמצעות מאפיין `badgeConfig`. תגים הם אינדיקטורים חזותיים שמופיעים ליד שם המשתמש בתגובות.

- `badgeIds` - מערך של מזהי תגים להקצאה למשתמש. אלה חייבים להיות מזהי תגים חוקיים שנוצרו בחשבון FastComments שלך. מוגבל ל-30 תגים.
- `override` - אם true, כל התגים הקיימים המוצגים בתגובות יוחלפו באלה שסופקו. אם false או לא מצוין, התגים שסופקו יתווספו לכל תגים קיימים.
- `update` - אם true, מאפייני תצוגת התגים יעודכנו מקונפיגורציית השוכר בכל פעם שהמשתמש מתחבר.
