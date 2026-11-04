המבנה היחיד שנשלח באמצעות webhooks הוא אובייקט WebhookComment, המתואר ב-TypeScript להלן.

#### מבנה אובייקט ה-WebhookComment

##### מבנה אירוע ה-"Create"
גוף הבקשה של אירוע ה-"create" הוא אובייקט WebhookComment.

##### מבנה אירוע ה-"Update"
גוף הבקשה של אירוע ה-"update" הוא אובייקט WebhookComment.

##### מבנה אירוע ה-"Delete"
גוף הבקשה של אירוע ה-"delete" הוא אובייקט WebhookComment.

    שינוי מתאריך 14 בנובמבר 2023
    בעבר גוף הבקשה של אירוע ה-"delete" הכיל רק את ה-id של התגובה. כעת הוא מכיל את התגובה המלאה במועד המחיקה.

כל מפתח תמיד קיים בגוף הבקשה. כאשר לתגובה אין ערך עבור שדה מסוים, הגוף מכיל `null`
(או `false` עבור בוליאנים ו-`[]` עבור רשימות), כך שהצורה של כל מסירה אינה משתנה מתגובה לתגובה.

[inline-code-attrs-start title = 'אובייקט ה-WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

כאשר משתמשים מתויגים בתגובה, המידע נשמר ברשימה בשם `mentions`. לכל אובייקט ברשימה זו
יש את המבנה הבא.

[inline-code-attrs-start title = 'אובייקט ה-Mentions של ה-Webhook'; type = 'typescript'; inline-code-attrs-end]
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

לכל רשומה ברשימת ה-`badges` יש את המבנה הבא. ראו את [מדריך ה-Badges](/guide-badges.html) לסוגי התגים.

[inline-code-attrs-start title = 'אובייקט ה-Badge של ה-Webhook'; type = 'typescript'; inline-code-attrs-end]
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

- **אירוע Create**: POST או PUT (ברירת מחדל: PUT)
- **אירוע Update**: POST או PUT (ברירת מחדל: PUT)
- **אירוע Delete**: DELETE, POST, או PUT (ברירת מחדל: DELETE)

מכיוון שכל הבקשות מכילות מזהה, פעולות ה-Create וה-Update הן אידמפוטנטיות כברירת מחדל (PUT). חזרה על אותה בקשת Create או Update לא אמורה ליצור אובייקטים כפולים בצד שלכם.

#### כותרות בקשה

כל בקשת webhook כוללת את הכותרות הבאות:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

ראו [אבטחה וטוקני API](/guide-webhooks.html#webhooks-api-tokens) למידע על אימות חתימת ה-HMAC.