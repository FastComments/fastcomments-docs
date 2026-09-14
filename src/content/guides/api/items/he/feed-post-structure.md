A `FeedPost` object מייצג פוסט בפיד של FastComments. פיד הוא זרם של פוסטים עם שרשורי תגובות משלהם, המוצגים על‑ידי ווידג'ט ה‑Feed. לכל פוסט יש מחבר, תוכן עשיר אופציונלי, מדיה וקישורים, וניתן לתייג אותו כך שניתן לסנן את הפיד.

המבנה של האובייקט `FeedPost` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** קריאה בלבד **/
    _id: string
    /** קריאה בלבד **/
    tenantId: string
    title?: string
    /** מזהה של משתמש FastComments או SSO שכתב את הפוסט. **/
    fromUserId?: string
    /** ממולא מהמשתמש כאשר לא מוגדר. **/
    fromUserDisplayName?: string | null
    /** קריאה בלבד. ממולא מהמשתמש. **/
    fromUserAvatar?: string | null
    /** משמש לסינון פיד. **/
    tags?: string[]
    /** משקל מיון בתוך פיד. ערכים גבוהים יותר ממוקמים ראשונים. **/
    weight?: number
    /** זוגות מפתח/ערך חופשיים לשימושך. **/
    meta?: Record<string, string>
    /** HTML מנוקה. **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** קריאה בלבד **/
    createdAt: string
    /** קריאה בלבד. סוג תגובה לספירה. **/
    reacts?: Record<string, number>
    /** קריאה בלבד **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** הכתובת שאליה מקושר פריט המדיה כאשר נלחץ. **/
    linkUrl?: string
    /** ערך אחד לכל גרסה. הווידג'ט בוחר את המתאימה ביותר. **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** טקסט הקישור, כגון "הירשם עכשיו". **/
    text?: string
    /** כותרת שמוצגת עם הקישור. **/
    title?: string
    /** תיאור שמוצג עם הקישור. **/
    description?: string
    url?: string
}
[inline-code-end]

הערות:

- חלק מהשדות מסומנים כ-`READONLY` - הם מוחזרים על ידי ה-API אך לא ניתן להגדירם.  
- התגובות על פוסט הן תגובות רגילות שה-`urlId` שלהן הוא `post:` ולאחריו מזהה הפוסט `_id`. השתמשו בערך זה עם API התגובות כדי לקרוא או ליצור תגובות על פוסט.