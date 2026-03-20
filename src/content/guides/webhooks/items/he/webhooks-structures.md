המבנה היחיד שנשלח דרך webhooks הוא האובייקט WebhookComment, המתואר ב-TypeScript למטה.

#### מבנה אובייקט WebhookComment

##### מבנה האירוע "create"
גוף הבקשה של אירוע "create" הוא אובייקט WebhookComment.

##### מבנה האירוע "update"
גוף הבקשה של אירוע "update" הוא אובייקט WebhookComment.

##### מבנה האירוע "delete"
גוף הבקשה של אירוע "delete" הוא אובייקט WebhookComment.

    שינוי מתאריך 14 בנובמבר 2023
    בעבר גוף הבקשה של אירוע "delete" הכיל רק את מזהה ההערה. כעת הוא מכיל את ההערה המלאה בזמן המחיקה.


[inline-code-attrs-start title = 'אובייקט WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** המזהה של ההערה. **/
    id: string
    /** המזהה או ה-URL שמזהה את שרשור ההערות. מנורמל. **/
    urlId: string
    /** ה-URL שמצביע על המקום שבו הושארה ההערה. **/
    url?: string
    /** המזהה של המשתמש שהשאיר את ההערה. אם SSO, מקדים אותו ב-tenant id. **/
    userId?: string
    /** הדוא"ל של המשתמש שהשאיר את ההערה. **/
    commenterEmail?: string
    /** השם של המשתמש שמוצג בווידג'ט ההערות. עם SSO, יכול להיות displayName. **/
    commenterName: string
    /** הטקסט הגולמי של ההערה. **/
    comment: string
    /** הטקסט של ההערה לאחר ניתוח. **/
    commentHTML: string
    /** מזהה חיצוני של ההערה. **/
    externalId?: string
    /** המזהה של ההערה האב. **/
    parentId?: string | null
    /** התאריך ב-UTC כאשר הושארה ההערה. **/
    date: UTC_ISO_DateString
    /** הקארמה המשולבת (up - down) של ההצבעות. **/
    votes: number
    votesUp: number
    votesDown: number
    /** true אם המשתמש היה מחובר כשפרסם את ההערה, או אם אימת את ההערה, או אם אימת את המפגש שלו כשההערה הושארה. **/
    verified: boolean
    /** התאריך בו ההערה אומתה. **/
    verifiedDate?: number
    /** אם ממדרטור סומנה ההערה כ"נבדקה". **/
    reviewed: boolean
    /** המיקום, או הקידוד base64, של האווטאר. יהיה base64 רק אם זה הערך שנשלח עם SSO. **/
    avatarSrc?: string
    /** האם ההערה סומנה כספאם באופן ידני או אוטומטי? **/
    isSpam: boolean
    /** האם ההערה סומנה כספאם באופן אוטומטי? **/
    aiDeterminedSpam: boolean
    /** האם יש תמונות בהערה? **/
    hasImages: boolean
    /** מספר העמוד שבו נמצאת ההערה עבור כיוון המיון "הכי רלוונטי". **/
    pageNumber: number
    /** מספר העמוד שבו נמצאת ההערה עבור כיוון המיון "הישנים ביותר תחילה". **/
    pageNumberOF: number
    /** מספר העמוד שבו נמצאת ההערה עבור כיוון המיון "החדשים ביותר תחילה". **/
    pageNumberNF: number
    /** האם ההערה אושרה באופן אוטומטי או ידני? **/
    approved: boolean
    /** קוד השפה/לוקל (פורמט: en_us) של המשתמש כאשר נכתבה ההערה. **/
    locale: string
    /** ה-@mentions שנכתבו בהערה ונותחו בהצלחה. **/
    mentions?: CommentUserMention[]
    /** הדומיין שממנו ההערה. **/
    domain?: string
    /** רשימה אופציונלית של מזהי קבוצות המודרציה המשויכים להערה זו. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

כאשר משתמשים מתוייגים בהערה, המידע מאוחסן ברשימה שנקראת `mentions`. כל אובייקט ברשימה זו
יש את המבנה הבא.

[inline-code-attrs-start title = 'אובייקט הזכרת משתמשים של Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** מזהה המשתמש. עבור משתמשי SSO, יהיה מקודם ב-tenant id שלכם. **/
    id: string
    /** הטקסט הסופי של תגית ה-@mention, כולל סמל ה-@. **/
    tag: string
    /** הטקסט המקורי של תגית ה-@mention, כולל סמל ה-@. **/
    rawTag: string
    /** איזה סוג משתמש זוהה בתג. user = חשבון FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** אם המשתמש בחר שלא לקבל התראות, זה עדיין יוגדר כ-true. **/
    sent: boolean
}
[inline-code-end]

#### שיטות HTTP

אתה יכול להגדיר את שיטת ה-HTTP לכל סוג אירוע webhook בלוח הניהול:

- **Create Event**: POST או PUT (ברירת מחדל: PUT)
- **Update Event**: POST או PUT (ברירת מחדל: PUT)
- **Delete Event**: DELETE, POST, או PUT (ברירת מחדל: DELETE)

מכיוון שכל הבקשות מכילות מזהה, פעולות Create ו-Update הן אידמופטנטיות כברירת מחדל (PUT). חזרה על אותה בקשת Create או Update לא אמורה ליצור עצמים כפולים אצלכם.

#### כותרות בקשה

כל בקשת webhook כוללת את הכותרות הבאות:

| Header | תיאור |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | סוד ה-API שלך |
| `X-FastComments-Timestamp` | חותמת זמן של Unix (שניות) כאשר הבקשה נחתמה |
| `X-FastComments-Signature` | חתימת HMAC-SHA256 (`sha256=<hex>`) |

ראו [אבטחה וטוקנים של API](/guide-webhooks.html#webhooks-api-tokens) למידע על אימות חתימת HMAC.