המבנה היחיד שנשלח דרך webhooks הוא אובייקט WebhookComment, המתואר ב-TypeScript להלן.

#### מבנה אובייקט WebhookComment

##### מבנה האירוע "create"
גוף הבקשה של אירוע "create" הוא אובייקט WebhookComment.

##### מבנה האירוע "update"
גוף הבקשה של אירוע "update" הוא אובייקט WebhookComment.

##### מבנה האירוע "delete"
גוף הבקשה של אירוע "delete" הוא אובייקט WebhookComment.

    שינוי מ-14 בנובמבר 2023
    בעבר גוף הבקשה של אירוע "delete" הכיל רק את ה-id של ההערה. כעת הוא מכיל את ההערה המלאה בזמן המחיקה.


[inline-code-attrs-start title = 'אובייקט WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ה-id של ההערה. **/
    id: string
    /** ה-id או ה-URL שמזהה את שרשור ההערות. מנורמל. **/
    urlId: string
    /** ה-URL שמצביע לאן הושארה ההערה. **/
    url?: string
    /** מזהה המשתמש שהשאיר את ההערה. אם SSO, מקודם במזהה ה-tenant. **/
    userId?: string
    /** הדוא"ל של המשתמש שהשאיר את ההערה. **/
    commenterEmail?: string
    /** השם של המשתמש שמוצג בווידג'ט ההערות. ב-SSO, יכול להיות displayName. **/
    commenterName: string
    /** הטקסט הגולמי של ההערה. **/
    comment: string
    /** טקסט ההערה לאחר עיבוד/ניתוח. **/
    commentHTML: string
    /** מזהה חיצוני של ההערה. **/
    externalId?: string
    /** ה-id של ההערה האב. **/
    parentId?: string | null
    /** התאריך ב-UTC כאשר הושארה ההערה. **/
    date: UTC_ISO_DateString
    /** קרמה משולבת של ההצבעות (up - down). **/
    votes: number
    votesUp: number
    votesDown: number
    /** true אם המשתמש היה מחובר כשכתב את ההערה, או אם אימת את ההערה, או אם אימת את המפגש שלו בעת כתיבת ההערה. **/
    verified: boolean
    /** התאריך שבו ההערה אומתה. **/
    verifiedDate?: number
    /** האם מודרטור סמן את ההערה כנסקרת. **/
    reviewed: boolean
    /** המיקום, או קידוד base64, של האווטאר. יהיה רק ב-base64 אם זו הייתה הערך שנשלח ב-SSO. **/
    avatarSrc?: string
    /** האם ההערה סומנה כספאם באופן ידני או אוטומטי? **/
    isSpam: boolean
    /** האם ההערה סומנה כספאם באופן אוטומטי? **/
    aiDeterminedSpam: boolean
    /** האם יש תמונות בהערה? **/
    hasImages: boolean
    /** מספר העמוד שבו ההערה נמצאת עבור מיון "Most Relevant". **/
    pageNumber: number
    /** מספר העמוד שבו ההערה נמצאת עבור מיון "Oldest First". **/
    pageNumberOF: number
    /** מספר העמוד שבו ההערה נמצאת עבור מיון "Newest First". **/
    pageNumberNF: number
    /** האם ההערה אושרה באופן אוטומטי או ידני? **/
    approved: boolean
    /** קוד המקומיות (פורמט: en_us) של המשתמש בעת כתיבת ההערה. **/
    locale: string
    /** ה-@mentions שנכתבו בהערה ונפענחו בהצלחה. **/
    mentions?: CommentUserMention[]
    /** הדומיין שממנו ההערה. **/
    domain?: string
    /** רשימה אופציונלית של מזהי קבוצות המודרציה הקשורות להערה זו. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

כאשר משתמשים מתויגים בהערה, המידע נשמר ברשימה הנקראת `mentions`. כל אובייקט ברשימה זו
בעל המבנה הבא.

[inline-code-attrs-start title = 'אובייקט Mentions של Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** מזהה המשתמש. עבור משתמשי SSO, יהיה מקודם במזהה ה-tenant שלכם. **/
    id: string
    /** טקסט תג ה-@ הסופי, כולל הסימן @. **/
    tag: string
    /** טקסט תג ה-@ המקורי, כולל הסימן @. **/
    rawTag: string
    /** סוג המשתמש שהתויג. user = חשבון FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** אם המשתמש מבטל קבלת התראות, זה עדיין יוגדר כ-true. **/
    sent: boolean
}
[inline-code-end]

#### שיטות HTTP בשימוש

**שתי הפעולות Create ו-Update משתמשות ב-HTTP PUT ולא ב-POST!**

מכיוון שכל הבקשות שלנו מכילות מזהה (ID), חזרה על אותה בקשת Create או Update לא צריכה ליצור אובייקטים חדשים בצד שלכם.

זה אומר שהקריאות הללו איידמפוטנטיות (idempotent) ויש לבצע אותן כ-PUT בהתאם למפרט HTTP.

---