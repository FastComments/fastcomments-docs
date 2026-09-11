The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### מבנה אובייקט WebhookComment

##### מבנה אירוע "Create"
The "create" event request body is a WebhookComment object.

##### מבנה אירוע "Update"
The "update" event request body is a WebhookComment object.

##### מבנה אירוע "Delete"
The "delete" event request body is a WebhookComment object.

    שינוי החל מ-14 בנובמבר 2023
    בעבר, גוף הבקשה של אירוע "delete" כלל רק את מזהה ההערה. כעת הוא כולל את ההערה המלאה בזמן המחיקה.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'אובייקט WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** מזהה ההערה. **/
    id: string
    /** המזהה או ה-URL שמזהים את שרשרת ההערות. מנורמל. **/
    urlId: string
    /** ה-URL שמצביע על המקום שבו הושארה ההערה. **/
    url: string | null
    /** מזהה המשתמש שהשאיר את ההערה. אם SSO, מקדים במזהה השוכר. **/
    userId: string | null
    /** כתובת האימייל של המשתמש שהשאיר את ההערה. **/
    commenterEmail: string | null
    /** שם המשתמש שמופיע בווידג'ט ההערה. עם SSO, יכול להיות displayName. **/
    commenterName: string
    /** טקסט ההערה הגולמי. **/
    comment: string
    /** טקסט ההערה לאחר ניתוח. **/
    commentHTML: string
    /** מזהה חיצוני של ההערה. **/
    externalId: string | null
    /** מזהה ההערה ההורה. **/
    parentId: string | null
    /** תאריך ה-UTC שבו הושארה ההערה. **/
    date: UTC_ISO_DateString
    /** קארמה משולבת (up - down) של ההצבעות. **/
    votes: number
    votesUp: number
    votesDown: number
    /** אמת אם המשתמש היה מחובר כאשר הוא הגיב, או שהאמת את ההערה, או אם הוא אימת את ההפעלה שלו כאשר ההערה נכתבה. **/
    verified: boolean
    /** תאריך ה-UTC שבו האמתה ההערה. **/
    verifiedDate: UTC_ISO_DateString | null
    /** אם מודרטור סימן שההערה נבדקה. **/
    reviewed: boolean
    /** המיקום, או קידוד base64, של האווטר. יהיה base64 רק אם זו הייתה הערך שהועבר עם SSO. **/
    avatarSrc: string | null
    /** האם ההערה סומנה כספאם באופן ידני או אוטומטי? **/
    isSpam: boolean
    /** האם ההערה סומנה כספאם באופן אוטומטי? **/
    aiDeterminedSpam: boolean
    /** האם יש תמונות בהערה? **/
    hasImages: boolean
    /** מספר העמוד שבו נמצאת ההערה עבור מיון "Most Relevant". **/
    pageNumber: number | null
    /** מספר העמוד שבו נמצאת ההערה עבור מיון "Oldest First". **/
    pageNumberOF: number | null
    /** מספר העמוד שבו נמצאת ההערה עבור מיון "Newest First". **/
    pageNumberNF: number | null
    /** האם ההערה אושרה באופן אוטומטי או ידני? **/
    approved: boolean
    /** קוד השפה (פורמט: en_us) של המשתמש כאשר נכתבה ההערה. **/
    locale: string | null
    /** ה-@mentions שנכתבו בהערה והפוענחו בהצלחה. ריק כאשר אין כאלה. **/
    mentions: CommentUserMention[]
    /** הדומיין שממנו ההערה. **/
    domain: string | null
    /** מזהי קבוצות המודרציה המשויכים להערה זו. ריק כאשר אין כאלה. **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'אובייקט ה-Mentions של Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** מזהה המשתמש. עבור משתמשי SSO, יתווסף לפניו מזהה השוכר שלך. **/
    id: string
    /** טקסט תגית @mention הסופי, כולל סימן @. **/
    tag: string
    /** טקסט תגית @mention המקורי, כולל סימן @. **/
    rawTag: string
    /** סוג המשתמש שסומן. user = חשבון FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** אם המשתמש בחר לא לקבל התראות, ערך זה עדיין יוגדר ל-true. **/
    sent: boolean
}
[inline-code-end]

#### שיטות HTTP

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### כותרות בקשה

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [אבטחה & אסימוני API](/guide-webhooks.html#webhooks-api-tokens) for מידע על אימות חתימת HMAC.