The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### The WebhookComment Object Structure

##### The "Create" Event Structure  
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure  
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure  
The "delete" event request body is a WebhookComment object.

Change as of Nov 14th 2023  
Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Every key is always present in the body. When the comment has no value for a field the body carries `null` (or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'אובייקט WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    /** מזהה ההערה. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    /** מזהה או כתובת URL שמזהים את שרשרת ההערות. מנורמל. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    /** כתובת ה-URL שמצביעה על המקום שבו נכתבה ההערה. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    /** מזהה המשתמש שהשאיר את ההערה. אם SSO, מקדים במזהה השוכר. **/
    userId: string | null
    /** The email of the user left the comment. **/
    /** כתובת האימייל של המשתמש שהשאיר את ההערה. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    /** שם המשתמש שמופיע בווידג'ט ההערה. עם SSO, יכול להיות displayName. **/
    commenterName: string
    /** Raw comment text. **/
    /** טקסט ההערה הגולמי. **/
    comment: string
    /** Comment text after parsing. **/
    /** טקסט ההערה לאחר עיבוד. **/
    commentHTML: string
    /** Comment external id. **/
    /** מזהה חיצוני של ההערה. **/
    externalId: string | null
    /** The id of the parent comment. **/
    /** מזהה של ההערה ההורה. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    /** תאריך UTC שבו נכתבה ההערה. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    /** קארמה משולבת (העלאה - הורדה) של ההצבעות. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    /** אמת אם המשתמש היה מחובר כאשר הוא הגיב, או שהאמת את ההערה, או אם הוא אימת את ההפעלה שלו כאשר נכתבה ההערה. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    /** תאריך UTC שבו האמת ההערה. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    /** אם מודרטור סימן שההערה נבדקה. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    /** המיקום, או קידוד base64, של האווטר. יהיה base64 רק אם זו הייתה הערך שהועבר עם SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    /** האם ההערה סומנה כספאם באופן ידני או אוטומטי? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    /** האם ההערה סומנה כספאם באופן אוטומטי? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    /** האם יש תמונות בהערה? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    /** מספר העמוד שבו נמצאת ההערה עבור מיון "הכי רלוונטי". **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    /** מספר העמוד שבו נמצאת ההערה עבור מיון "הישן ביותר ראשון". **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    /** מספר העמוד שבו נמצאת ההערה עבור מיון "החדש ביותר ראשון". **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    /** האם ההערה אושרה באופן אוטומטי או ידני? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    /** קוד השפה (פורמט: en_us) של המשתמש כאשר נכתבה ההערה. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    /** ה-@mentions שנכתבו בהערה והפוענחו בהצלחה. ריק כאשר אין כאלה. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    /** הדומיין שממנו הגיעה ההערה. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    /** מזהי קבוצות המודרציה המשויכים להערה זו. ריק כאשר אין כאלה. **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list has the following structure.

[inline-code-attrs-start title = 'אובייקט ה-Mentions של Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    /** מזהה המשתמש. עבור משתמשי SSO, יהיה מקדים במזהה השוכר שלך. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    /** טקסט תגית @mention הסופי, כולל סימן @. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    /** טקסט תגית @mention המקורי, כולל סימן @. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    /** סוג המשתמש שסומן. user = חשבון FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    /** אם המשתמש בחר לא לקבל התראות, זה עדיין יוגדר כ-true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST or PUT (default: PUT)  
  **אירוע יצירה**: POST או PUT (ברירת מחדל: PUT)
- **Update Event**: POST or PUT (default: PUT)  
  **אירוע עדכון**: POST או PUT (ברירת מחדל: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)  
  **אירוע מחיקה**: DELETE, POST, או PUT (ברירת מחדל: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.  
מאחר שכל הבקשות מכילות מזהה, פעולות יצירה ועדכון הן אידמפוטנטיות כברירת מחדל (PUT). חזרה על אותה בקשת יצירה או עדכון לא צריכה ליצור אובייקטים משוכפלים בצד שלכם.

#### Request Headers

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `token` | סוד ה-API שלך |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Timestamp` | חותמת זמן Unix (שניות) כאשר הבקשה נחתמה |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |
| `X-FastComments-Signature` | חתימת HMAC-SHA256 (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.  
ראו [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) למידע על אימות חתימת HMAC.