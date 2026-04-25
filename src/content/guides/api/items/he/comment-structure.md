אובייקט `Comment` מייצג תגובה שהשאיר משתמש.

הקשר בין תגובות אב וצאצא מוגדר באמצעות `parentId`.

המבנה של אובייקט Comment הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה תגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** לקריאה בלבד: יוגדר ל-true אם מנוע הספאם קבע שהתגובה היא ספאם. **/
    aiDeterminedSpam?: boolean
    /** האם התגובה מאושרת להצגה. תוגדר ל-true בעת שמירת התגובה; אחרת היא תוסתר. **/
    approved?: boolean
    /** האווטאר של המשתמש. **/
    avatarSrc?: string
    /** Child comments. Not populated in all scenarios. Used when asTree is set to true via the API. **/
    children: Comment[]
    /** הטקסט הגולמי של התגובה. **/
    comment: string
    /** לקריאה בלבד: הטקסט של התגובה לאחר פירוס ל-HTML. **/
    commentHTML?: string
    /** האימייל של הכותב. נדרש אם תגובות אנונימיות כבויות. **/
    commenterEmail?: string
    /** הקישור של הכותב (למשל הבלוג שלו). **/
    commenterLink?: string
    /** השם של הכותב. תמיד נדרש. אם לא זמין, קבע ערך כמו "Anonymous". **/
    commenterName: string
    /** התאריך שבו נכתבה התגובה, ביחידת זמן epoch UTC. **/
    date: number
    /** התווית לתצוגה של התגובה - לדוגמה "Admin", "Moderator", או משהו כמו "VIP User". **/
    displayLabel?: string
    /** הדומיין שבו פורסמה התגובה. **/
    domain?: string
    /** לקריאה בלבד: מספר הפעמים שהתגובה סומנה. **/
    flagCount?: number
    /** ההאשטאגים (#) שנכתבו בתגובה ופוענחו בהצלחה. ניתן גם להוסיף האשטאגים ידנית לצורך שאילתות, אך הם לא יוצגו אוטומטית בטקסט התגובה. **/
    hashTags?: CommentHashTag[]
    /** לקריאה בלבד: האם התגובה מכילה תמונות? **/
    hasImages?: boolean
    /** לקריאה בלבד: האם התגובה מכילה קישורים? **/
    hasLinks?: boolean
    /** לקריאה בלבד: המזהה הייחודי של התגובה. **/
    id: string
    /** רק בעת יצירה! ערך זה מעובד ב-hash לצורך אחסון. **/
    ip?: string
    /** לקריאה בלבד: האם המשתמש הנוכחי חסם את מי שכתב את התגובה? **/
    isBlocked?: boolean
    /** לקריאה בלבד: האם התגובה נכתבה על ידי אדמין? מוגדר אוטומטית על בסיס userId. **/
    isByAdmin?: boolean
    /** לקריאה בלבד: האם התגובה נכתבה על ידי מודראטור? מוגדר אוטומטית על בסיס userId. **/
    isByModerator?: boolean
    /** יוגדר ל-true אם התגובה נמחקה באופן רך (soft deleted) — הושאר ממלא מקום בשל קונפיגורציה אחרת. **/
    isDeleted?: boolean
    /** יוגדר ל-true אם חשבון המשתמש נמחק והתגובה נדרשה להישאר. **/
    isDeletedUser?: boolean
    /** לקריאה בלבד: האם התגובה סומנה על ידי המשתמש המחובר כרגע (contextUserId)? **/
    isFlagged?: boolean
    /** האם התגובה מוצמדת (pinned)? **/
    isPinned?: boolean
    /** האם התגובה נעולה? כאשר true, אף אחד (כולל מודראטורים) לא יכול להשיב, לערוך או למחוק אותה עד שתיפתח. **/
    isLocked?: boolean
    /** האם התגובה היא ספאם? **/
    isSpam?: boolean
    /** לקריאה בלבד: האם המשתמש הנוכחי (contextUserId) הצביע נגד התגובה? **/
    isVotedDown?: boolean
    /** לקריאה בלבד: האם המשתמש הנוכחי (contextUserId) הצביע בעד התגובה? **/
    isVotedUp?: boolean
    /** השפה/לוקל של התגובה. אם לא סופק, היא נגזרת מכותרת ה-HTTP שמציינת את שפות ההעדפה. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** לקריאה בלבד: ה-@mentions שנכתבו בתגובה ופוענחו בהצלחה. **/
    mentions?: CommentUserMention[]
    /** מטה-נתונים אופציונליים המשויכים לתגובה. **/
    meta?: Record<string, string | number | boolean>
    /** The optional list of moderation group ids associated with this comment. **/
    moderationGroupIds?: string[]|null
    /** לקריאה בלבד: המזהה של אובייקט ההצבעה המתאים להצבעת המשתמש הנוכחי (contextUserId) על תגובה זו. **/
    myVoteId?: string
    /** האם נשלחו התראות על תגובה זו למגיבים. כדי למנוע שליחת התראות בייבוא, קבע ערך זה ל-true. **/
    notificationSentForParent?: boolean
    /** האם נשלחו התראות על תגובה זו למשתמשי ה-tenant. כדי למנוע שליחת התראות בייבוא, קבע ל-true. **/
    notificationSentForParentTenant?: boolean
    /** הכותרת של העמוד שבו הופיעה התגובה. **/
    pageTitle?: string
    /** אם אנו משיבים לתגובה, זהו המזהה שאליו משיבים. **/
    parentId?: string|null
    /** האם התגובה מסומנת כנגועה בבדיקה (reviewed). **/
    reviewed: boolean
    /** מזהה ה-tenant שאליו שייכת התגובה. **/
    tenantId: string
    /** המשתמש שכתב את התגובה. נוצר אוטומטית כאשר שומרים תגובה שכוללת שם/אימייל. **/
    userId?: string|null
    /** ה-URL למיקום שבו התגובה נראית, לדוגמה פוסט בבלוג. **/
    url: string
    /** גרסה "נוקה" של ה-urlId שסיפקת. בעת שמירה, אתה מציין שדה זה, אך כאשר משחזרים את התגובה הוא יעובד והערך המקורי יועבר ל-"urlIdRaw". **/
    urlId: string
    /** לקריאה בלבד: ה-urlId המקורי שסיפקת. **/
    urlIdRaw?: string
    /** האם המשתמש והתגובה מאומתים? **/
    verified: boolean
    /** מספר ההצבעות בעד. **/
    votesUp?: number
    /** מספר ההצבעות נגד. **/
    votesDown?: number
    /** "הקרמה" של התגובה (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

חלק מהשדות הללו מסומנים כ-`READONLY` - אלה מוחזרים על ידי ה-API אך לא יכולים להיות מוגדרים.

### Comment Text Structure

תגובות נכתבות בגירסה של Markdown שמותאמת ל-FastComments, שהיא Markdown בתוספת תגים בסגנון `bbcode` עבור תמונות, כמו `[img]path[/img]`.

הטקסט נשמר בשני שדות. הטקסט שהמשתמש הזין נשמר ללא שינוי בשדה `comment`. זה מורנדר ונשמר בשדה `commentHTML`.

תגי ה-HTML המותרים הם `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

מומלץ להציג את ה-HTML, מכיוון שמדובר בתת-קבוצה מאוד קטנה של HTML, ובניית מנרדרר היא יחסית פשוטה. קיימות ספריות רבות עבור React Native ו-Flutter, למשל, שעוזרות בכך.

אתה רשאי לבחור להציג את הערך הלא-נורמליזציוני של השדה `comment`. [An example parser is here.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

ניתן גם להתאים את מפענח הדוגמה לעבודה עם HTML, ולהמיר את תווי ה-HTML לאלמנטים הצפויים להצגה בפלטפורמה שלך.

### Tagging

כאשר משתמשים מסומנים בתגובה, המידע מאוחסן ברשימה שנקראת `mentions`. כל אובייקט ברשימה זו
יש את המבנה הבא.

[inline-code-attrs-start title = 'אובייקט אזכורי התגובה'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** מזהה המשתמש. עבור משתמשי SSO, יהיה עם קידומת של מזהה ה-tenant שלכם. **/
    id: string
    /** הטקסט הסופי של תגית ה-@mention, כולל הסימן @. **/
    tag: string
    /** הטקסט המקורי של ה-@mention, כולל הסימן @. **/
    rawTag: string
    /** סוג המשתמש שהוזכר. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** גם אם המשתמש מבטל קבלת התראות, ערך זה עדיין יהיה true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

כאשר משתמשים בהאשטאגים והם פוענחו בהצלחה, המידע מאוחסן ברשימה שנקראת `hashTags`. כל אובייקט ברשימה זו
יש את המבנה הבא. ניתן גם להוסיף האשטאגים ידנית למערך `hashTags` של התגובה לצורך שאילתות, אם `retain` מוגדר.

[inline-code-attrs-start title = 'אובייקט האשטאג של התגובה'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** המזהה של ההאשטאג. **/
    id: string
    /** הטקסט הסופי של תגית ה-#hashtag, כולל הסימן #. **/
    tag: string
    /** אם לאשטאג מקושר ל-URL מותאם אישית, זה יהיה מוגדר. **/
    url?: string
    /** האם לשמור את ההאשטאג גם אם הוא לא קיים בטקסט התגובה כאשר התגובה מעודכנת. שימושי לתיוג תגובות מבלי לשנות את טקסט התגובה. **/
    retain?: boolean
}
[inline-code-end]

---