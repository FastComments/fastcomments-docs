המבנה היחיד שנשלח דרך webhooks הוא האובייקט WebhookComment, המתואר ב-TypeScript למטה.

#### מבנה ה-WebhookComment

##### מבנה האירוע "create"
גוף הבקשה של אירוע ה-"create" הוא אובייקט WebhookComment.

##### מבנה האירוע "update"
גוף הבקשה של אירוע ה-"update" הוא אובייקט WebhookComment.

##### מבנה האירוע "delete"
גוף הבקשה של אירוע ה-"delete" הוא אובייקט WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'אובייקט WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ה-id של התגובה. **/
    id: string
    /** ה-id או ה-URL שמזהה את שרשור התגובות. מנורמל. **/
    urlId: string
    /** ה-URL המצביע היכן הושארה התגובה. **/
    url?: string
    /** ה-id של המשתמש שהשאיר את התגובה. אם SSO, יהיה עם קידומת מזהה ה-tenant. **/
    userId?: string
    /** הדוא"ל של המשתמש שהשאיר את התגובה. **/
    commenterEmail?: string
    /** השם של המשתמש שמופיע בווידג'ט התגובות. ב-SSO, יכול להיות displayName. **/
    commenterName: string
    /** טקסט התגובה הגולמי. **/
    comment: string
    /** טקסט התגובה לאחר עיבוד. **/
    commentHTML: string
    /** id חיצוני של התגובה. **/
    externalId?: string
    /** ה-id של תגובת האב. **/
    parentId?: string | null
    /** תאריך UTC שבו הושארה התגובה. **/
    date: UTC_ISO_DateString
    /** קארמה משולבת (up - down) של ההצבעות. **/
    votes: number
    votesUp: number
    votesDown: number
    /** true אם המשתמש היה מחובר כשהגיב, או אם אימת את התגובה, או אם אימת את הסשן שלו בעת השארת התגובה. **/
    verified: boolean
    /** התאריך שבו התגובה אומתה. **/
    verifiedDate?: number
    /** אם מודרטור סמן את התגובה כנבדקה. **/
    reviewed: boolean
    /** המיקום, או קידוד base64, של האוואטר. יהיה base64 רק אם זו הייתה הערך שנשלח ב-SSO. **/
    avatarSrc?: string
    /** האם התגובה סומנה כספאם ידנית או אוטומטית? **/
    isSpam: boolean
    /** האם התגובה סומנה כספאם באופן אוטומטי? **/
    aiDeterminedSpam: boolean
    /** האם יש תמונות בתגובה? **/
    hasImages: boolean
    /** מספר העמוד שבו נמצאת התגובה עבור כיוון המיון "Most Relevant". **/
    pageNumber: number
    /** מספר העמוד שבו נמצאת התגובה עבור כיוון המיון "Oldest First". **/
    pageNumberOF: number
    /** מספר העמוד שבו נמצאת התגובה עבור כיוון המיון "Newest First". **/
    pageNumberNF: number
    /** האם התגובה אושרה אוטומטית או ידנית? **/
    approved: boolean
    /** קוד השפה/אזור (פורמט: en_us) של המשתמש כאשר נכתבה התגובה. **/
    locale: string
    /** ה-@mentions שנכתבו בתגובה ונותחו בהצלחה. **/
    mentions?: CommentUserMention[]
    /** הדומיין שממנו הגיעה התגובה. **/
    domain?: string
    /** רשימה אופציונלית של מזהי קבוצות המודרציה המשויכים לתגובה זו. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'אובייקט המאזכרים של Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ה-id של המשתמש. עבור משתמשי SSO, יהיה עם קידומת מזהה ה-tenant שלכם. **/
    id: string
    /** הטקסט הסופי של תג ה-@, כולל סימן ה-@. **/
    tag: string
    /** הטקסט המקורי של תג ה-@, כולל סימן ה-@. **/
    rawTag: string
    /** סוג המשתמש שסומן. user = חשבון FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** אם המשתמש בחר לא לקבל התראות, ערך זה עדיין יוגדר כ-true. **/
    sent: boolean
}
[inline-code-end]

#### שיטות HTTP

ניתן להגדיר את שיטת ה-HTTP עבור כל סוג אירוע webhook בלוח הניהול:

- **אירוע Create**: POST או PUT (ברירת מחדל: PUT)
- **אירוע Update**: POST או PUT (ברירת מחדל: PUT)
- **אירוע Delete**: DELETE, POST, או PUT (ברירת מחדל: DELETE)

מאחר שכל הבקשות מכילות מזהה, פעולות Create ו-Update הן אידמפוטנטיות כברירת מחדל (PUT). חזרה על אותה בקשת Create או Update לא אמורה ליצור אובייקטים כפולים בצד שלכם.

#### כותרות הבקשה

כל בקשת webhook כוללת את הכותרות הבאות:

| כותרת | תיאור |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | סוד ה-API שלך |
| `X-FastComments-Timestamp` | חותמת זמן Unix (בשניות) כאשר הבקשה נחתמה |
| `X-FastComments-Signature` | חתימת HMAC-SHA256 (`sha256=<hex>`) |

ראה [אבטחה ומפתחות API](/guides/webhooks/webhooks-api-tokens) למידע על אימות חתימת HMAC.

---