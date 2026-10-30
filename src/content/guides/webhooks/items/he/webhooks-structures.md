The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### מבנה אובייקט WebhookComment

##### מבנה אירוע "Create"
The "create" event request body is a WebhookComment object.

##### מבנה אירוע "Update"
The "update" event request body is a WebhookComment object.

##### מבנה אירוע "Delete"
The "delete" event request body is a WebhookComment object.

שינוי החל מ-14 בנובמבר 2023  
Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'אובייקט WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

כאשר משתמשים מתוייגים בהערה, המידע נשמר ברשימה בשם `mentions`. לכל אובייקט ברשימה יש את המבנה הבא.

[inline-code-attrs-start title = 'אובייקט Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

לכל ערך ברשימת `badges` יש את המבנה הבא. ראו את [מדריך התגים](/guide-badges.html) עבור סוגי התגים.

[inline-code-attrs-start title = 'אובייקט Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### שיטות HTTP

ניתן להגדיר את שיטת ה-HTTP עבור כל סוג אירוע webhook בלוח הניהול:

- **אירוע יצירה**: POST או PUT (ברירת מחדל: PUT)
- **אירוע עדכון**: POST או PUT (ברירת מחדל: PUT)
- **אירוע מחיקה**: DELETE, POST, או PUT (ברירת מחדל: DELETE)

מאחר וכל הבקשות מכילות מזהה, פעולות יצירה ועדכון הן אידמפוטנטיות כברירת מחדל (PUT). חזרה על אותה בקשת יצירה או עדכון לא תיצור אובייקטים משוכפלים בצד שלכם.

#### כותרות בקשה

כל בקשת webhook כוללת את הכותרות הבאות:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | סוד ה-API שלך |
| `X-FastComments-Timestamp` | חותמת זמן Unix (שניות) כאשר הבקשה נחתמה |
| `X-FastComments-Signature` | חתימת HMAC-SHA256 (`sha256=<hex>`) |

ראו את [אבטחה & אסימוני API](/guide-webhooks.html#webhooks-api-tokens) למידע על אימות חתימת HMAC.

---