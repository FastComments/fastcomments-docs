`User` הוא אובייקט המייצג מכנה משותף של כל המשתמשים.

זכור שב-FastComments יש לנו מספר מקרי שימוש שונים למשתמשים:

- SSO מאובטח
- SSO פשוט
- משתמשי שוכר (לדוגמה: מנהלים)
- מגיבים

API זה מיועד ל**מגיבים** ולמשתמשים שנוצרו דרך **SSO פשוט**. בעצם, כל משתמש שנוצר
דרך האתר שלך ניתן לגישה דרך API זה. ניתן לאחזר גם משתמשי שוכר בדרך זו, אך תקבל מידע נוסף על ידי אינטראקציה עם ה-API `/tenant-users/`.

עבור `SSO מאובטח` אנא השתמש ב-API `/sso-users/`.

לא ניתן לעדכן סוגים אלה של משתמשים. הם יצרו את החשבון שלהם דרך האתר שלך, אז אנו מספקים גישה בסיסית לקריאה בלבד, אבל
לא ניתן לבצע שינויים. אם אתה רוצה לקבל זרימה כזו - אתה צריך להגדיר `SSO מאובטח`.

המבנה עבור אובייקט `User` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]
