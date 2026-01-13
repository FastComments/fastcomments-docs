אובייקט `Comment` מייצג תגובה שהושארה על ידי משתמש.

היחס בין תגובות הורה וילד מוגדר באמצעות `parentId`.

המבנה עבור אובייקט Comment הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Set to true if the spam engine determined the comment was spam. **/
    aiDeterminedSpam?: boolean
    /** Whether the comment is approved to show. Set to true when saving the comment, else it will be hidden. **/
    approved?: boolean
    /** The user's avatar. **/
    avatarSrc?: string
    /** Child comments. Not populated in all scenarios. Used when asTree is set to true via the API. **/
    children: Comment[]
    /** The commenter's raw comment. **/
    comment: string
    /** READONLY: The commenter's comment parsed into HTML. **/
    commentHTML?: string
    /** The commenter's email. Required if anonymous commenting is off. **/
    commenterEmail?: string
    /** The commenter's link (for example, their blog). **/
    commenterLink?: string
    /** The commenter's name. Always required. If not available, set to something like "Anonymous". **/
    commenterName: string
    /** The date the comment was left, in UTC epoch. **/
    date: number
    /** The "display label" for the comment - for example "Admin", "Moderator", or something like "VIP User". **/
    displayLabel?: string
    /** The domain the comment was posted on. **/
    domain?: string
    /** READONLY: The number of times the comment was flagged. **/
    flagCount?: number
    /** The #hashtags written in the comment that were successfully parsed. You can also manually add hashtags, for querying, but they won't display in the comment text automatically. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Does the comment contain images? **/
    hasImages?: boolean
    /** READONLY: Does the comment contain links? **/
    hasLinks?: boolean
    /** READONLY: The unique comment id. **/
    id: string
    /** Only on create! This is hashed for storage. **/
    ip?: string
    /** READONLY: Did the current user block the user that wrote this comment? **/
    isBlocked?: boolean
    /** READONLY: Is the comment by an admin? Automatically set based on userId. **/
    isByAdmin?: boolean
    /** READONLY: Is the comment by a moderator? Automatically set based on userId. **/
    isByModerator?: boolean
    /** Set to true if the comment was soft deleted (placeholder had to be left due to some other configuration). **/
    isDeleted?: boolean
    /** Set to true if the user's account was deleted and the comment had to be retained. **/
    isDeletedUser?: boolean
    /** READONLY: Is the flagged by the currently logged-in user (contextUserId)? **/
    isFlagged?: boolean
    /** Is the comment pinned? **/
    isPinned?: boolean
    /** Is the comment locked for new replies (moderators still can reply)? **/
    isLocked?: boolean
    /** Is the comment spam? **/
    isSpam?: boolean
    /** READONLY: Is the comment voted down for the current user (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Is the comment voted up for the current user (contextUserId)? **/
    isVotedUp?: boolean
    /** The locale the comment is in. If not provided, will be derived from the language accept HTTP header. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** Optional metadata associated with the comment. **/
    meta?: Record<string, string | number | boolean>
    /** The optional list of moderation group ids associated with this comment. **/
    moderationGroupIds?: string[]|null
    /** READONLY: The id of the vote object that corresponds to the vote from the current user (contextUserId) on this comment. **/
    myVoteId?: string
    /** Whether notifications were sent for this comment for commenters. To prevent notifications being sent on imports, set this to true. **/
    notificationSentForParent?: boolean
    /** Whether notifications were sent for this comment for tenant users. To prevent notifications being sent on imports, set this to true. **/
    notificationSentForParentTenant?: boolean
    /** The title of the page this comment was on. **/
    pageTitle?: string
    /** If we're replying to a comment, this is the ID that we are replying to. **/
    parentId?: string|null
    /** Whether the comment is marked reviewed. **/
    reviewed: boolean
    /** The tenant id where the comment belongs. **/
    tenantId: string
    /** The user that wrote the comment. Created automatically when saving a comment with a name/email. **/
    userId?: string|null
    /** The URL to the location that this comment is visible, like a blog post. **/
    url: string
    /** A "cleaned" version of the urlId you passed us. When saving, you specify this field, but when you fetch the comment back this will be "cleaned" and your original value moved to "urlIdRaw". **/
    urlId: string
    /** READONLY: The original urlId you passed us. **/
    urlIdRaw?: string
    /** Is the user and this comment verified? **/
    verified: boolean
    /** Number of votes up. **/
    votesUp?: number
    /** Number of votes down. **/
    votesDown?: number
    /** The "karma" of the comment (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

חלק מהשדות הללו מסומנים כ-`READONLY` - אלה מוחזרים על ידי ה-API אבל לא ניתן להגדיר אותם.

### מבנה טקסט התגובה

תגובות נכתבות בגרסת FastComments של markdown, שהיא פשוט markdown בתוספת תגיות בסגנון `bbcode` מסורתי לתמונות, כמו `[img]path[/img]`.

הטקסט מאוחסן בשני שדות. הטקסט שהמשתמש הזין מאוחסן ללא שינוי בשדה `comment`. זה מרונדר ומאוחסן בשדה `commentHTML`.

תגיות ה-HTML המותרות הן `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, ו-br`.

מומלץ לרנדר את ה-HTML, מכיוון שזו תת-קבוצה קטנה מאוד של HTML, בניית רנדרר היא די פשוטה. ישנן מספר ספריות ל-React Native ו-Flutter, לדוגמה, שעוזרות בכך.

אתה יכול לבחור לרנדר את הערך הלא-מנורמל של שדה `comment`. [דוגמת פרסר נמצאת כאן.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

ניתן גם להתאים את דוגמת הפרסר לעבודה עם HTML, ולהמיר את תגיות ה-HTML לאלמנטים צפויים לרנדור עבור הפלטפורמה שלך.

### תיוג

כאשר משתמשים מתויגים בתגובה, המידע מאוחסן ברשימה בשם `mentions`. לכל אובייקט ברשימה זו
יש את המבנה הבא.

[inline-code-attrs-start title = 'אובייקט אזכורי תגובה'; type = 'typescript'; inline-code-attrs-end]
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

### האשטאגים

כאשר האשטאגים משמשים ומפורסרים בהצלחה, המידע מאוחסן ברשימה בשם `hashTags`. לכל אובייקט ברשימה זו
יש את המבנה הבא. ניתן גם להוסיף האשטאגים באופן ידני למערך `hashTags` של התגובה לשאילתות, אם `retain` מוגדר.

[inline-code-attrs-start title = 'אובייקט האשטאג תגובה'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** The hashtag id. **/
    id: string
    /** The final #hashtag tag text, including the # symbol. **/
    tag: string
    /** If the hashtag is associated with a custom URL, this will be defined. **/
    url?: string
    /** If we should retain the hashtag, even if it does not exist in the comment text, when the comment is updated. Useful for tagging comments without changing comment text. **/
    retain?: boolean
}
[inline-code-end]
