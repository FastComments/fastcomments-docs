אובייקט `Comment` מייצג תגובה שנכתבה על ידי משתמש.

הקשר בין תגובות הורה לתגובות צאצא מוגדר באמצעות `parentId`.

המבנה של אובייקט ה-Comment הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה התגובה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: מוגדר כ-true אם מנוע הספאם קבע שהתגובה היא ספאם. **/
    aiDeterminedSpam?: boolean
    /** האם התגובה מאושרת להצגה. מוגדר כ-true בעת שמירת התגובה, אחרת היא תוסתר. **/
    approved?: boolean
    /** האווטאר של המשתמש. **/
    avatarSrc?: string
    /** תגובות צאצא. לא מאוכלס בכל התרחישים. בשימוש כאשר asTree מוגדר כ-true דרך ה-API. **/
    children: Comment[]
    /** התגובה הגולמית של המגיב. **/
    comment: string
    /** READONLY: התגובה של המגיב מפורסרת ל-HTML. **/
    commentHTML?: string
    /** האימייל של המגיב. נדרש אם תגובות אנונימיות כבויות. **/
    commenterEmail?: string
    /** הקישור של המגיב (למשל, הבלוג שלו). **/
    commenterLink?: string
    /** השם של המגיב. תמיד נדרש. אם אינו זמין, יש להגדיר למשהו כמו "Anonymous". **/
    commenterName: string
    /** התאריך שבו נכתבה התגובה, ב-UTC epoch. **/
    date: number
    /** "תווית התצוגה" של התגובה - למשל "Admin", "Moderator", או משהו כמו "VIP User". **/
    displayLabel?: string
    /** הדומיין שבו פורסמה התגובה. **/
    domain?: string
    /** READONLY: מספר הפעמים שהתגובה סומנה. **/
    flagCount?: number
    /** ה-#hashtags שנכתבו בתגובה ופורסרו בהצלחה. ניתן גם להוסיף hashtags ידנית, לצורך שאילתות, אך הם לא יוצגו בטקסט התגובה אוטומטית. **/
    hashTags?: CommentHashTag[]
    /** READONLY: האם התגובה מכילה תמונות? **/
    hasImages?: boolean
    /** READONLY: האם התגובה מכילה קישורים? **/
    hasLinks?: boolean
    /** READONLY: מזהה התגובה הייחודי. **/
    id: string
    /** רק ביצירה! זה מוצפן לאחסון. **/
    ip?: string
    /** READONLY: האם המשתמש הנוכחי חסם את המשתמש שכתב תגובה זו? **/
    isBlocked?: boolean
    /** READONLY: האם התגובה נכתבה על ידי מנהל? מוגדר אוטומטית לפי userId. **/
    isByAdmin?: boolean
    /** READONLY: האם התגובה נכתבה על ידי מנחה? מוגדר אוטומטית לפי userId. **/
    isByModerator?: boolean
    /** מוגדר כ-true אם התגובה נמחקה רך (היה צורך להשאיר placeholder עקב תצורה אחרת). **/
    isDeleted?: boolean
    /** מוגדר כ-true אם חשבון המשתמש נמחק והיה צורך לשמר את התגובה. **/
    isDeletedUser?: boolean
    /** READONLY: האם סומן על ידי המשתמש המחובר הנוכחי (contextUserId)? **/
    isFlagged?: boolean
    /** האם התגובה ננעצה? **/
    isPinned?: boolean
    /** האם התגובה נעולה? כאשר true, אף אחד (כולל מנחים) לא יכול להגיב, לערוך, או למחוק אותה עד שתיפתח. **/
    isLocked?: boolean
    /** האם התגובה היא ספאם? **/
    isSpam?: boolean
    /** READONLY: האם התגובה הוצבעה למטה עבור המשתמש הנוכחי (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: האם התגובה הוצבעה למעלה עבור המשתמש הנוכחי (contextUserId)? **/
    isVotedUp?: boolean
    /** הלוקל של התגובה. אם לא סופק, ייגזר מכותרת ה-HTTP accept language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: ה-@mentions שנכתבו בתגובה ופורסרו בהצלחה. **/
    mentions?: CommentUserMention[]
    /** מטא-דאטה אופציונלית המשויכת לתגובה. **/
    meta?: Record<string, string | number | boolean>
    /** הרשימה האופציונלית של מזהי קבוצות מודרציה המשויכים לתגובה זו. **/
    moderationGroupIds?: string[]|null
    /** READONLY: מזהה אובייקט ההצבעה המתאים להצבעה של המשתמש הנוכחי (contextUserId) על תגובה זו. **/
    myVoteId?: string
    /** האם נשלחו התראות עבור תגובה זו למגיבים. למניעת שליחת התראות בייבוא, הגדירו זאת כ-true. **/
    notificationSentForParent?: boolean
    /** האם נשלחו התראות עבור תגובה זו למשתמשי הטננט. למניעת שליחת התראות בייבוא, הגדירו זאת כ-true. **/
    notificationSentForParentTenant?: boolean
    /** הכותרת של הדף שבו הייתה התגובה. **/
    pageTitle?: string
    /** אם אנו מגיבים לתגובה, זהו המזהה שאליו אנו מגיבים. **/
    parentId?: string|null
    /** האם התגובה מסומנת כנבדקה. **/
    reviewed: boolean
    /** מזהה הטננט שאליו שייכת התגובה. **/
    tenantId: string
    /** המשתמש שכתב את התגובה. נוצר אוטומטית בעת שמירת תגובה עם שם/אימייל. **/
    userId?: string|null
    /** ה-URL למיקום שבו תגובה זו גלויה, כמו פוסט בבלוג. **/
    url: string
    /** גרסה "מנוקה" של ה-urlId שהעברתם לנו. בעת שמירה, אתם מציינים שדה זה, אך כאשר אתם מביאים את התגובה בחזרה זה יהיה "מנוקה" והערך המקורי שלכם יועבר ל-"urlIdRaw". **/
    urlId: string
    /** READONLY: ה-urlId המקורי שהעברתם לנו. **/
    urlIdRaw?: string
    /** האם המשתמש והתגובה הזו מאומתים? **/
    verified: boolean
    /** מספר ההצבעות למעלה. **/
    votesUp?: number
    /** מספר ההצבעות למטה. **/
    votesDown?: number
    /** ה-"קארמה" של התגובה (= הצבעות למעלה - הצבעות למטה). **/
    votes?: number
}
[inline-code-end]

חלק מהשדות הללו מסומנים כ-`READONLY` - אלה מוחזרים על ידי ה-API אך לא ניתנים להגדרה.

### מבנה טקסט התגובה

תגובות נכתבות בגרסת FastComments של markdown, שהיא פשוט markdown ועוד תגיות בסגנון `bbcode` מסורתי לתמונות, כמו `[img]path[/img]`.

הטקסט נשמר בשני שדות. הטקסט שהמשתמש הזין נשמר ללא שינוי בשדה `comment`. טקסט זה מרונדר ונשמר בשדה `commentHTML`.

תגיות ה-HTML המותרות הן `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

מומלץ לרנדר את ה-HTML, מכיוון שמדובר בתת-קבוצה קטנה מאוד של HTML, בניית רנדרר היא פשוטה יחסית. קיימות מספר ספריות עבור React Native ו-Flutter, למשל, שיכולות לסייע בכך.

אתם יכולים לבחור לרנדר את הערך הלא-מנורמל של שדה `comment`. [דוגמת פרסר נמצאת כאן.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

ניתן להתאים את פרסר הדוגמה גם לעבודה עם HTML, ולהמיר את תגיות ה-HTML לאלמנטים הצפויים לרנדור עבור הפלטפורמה שלכם.

### תיוג (Tagging)

כאשר משתמשים מתויגים בתגובה, המידע נשמר ברשימה בשם `mentions`. לכל אובייקט ברשימה זו
יש את המבנה הבא.

[inline-code-attrs-start title = 'אובייקט ה-Mentions של התגובה'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** מזהה המשתמש. עבור משתמשי SSO, זה יכלול את מזהה הטננט שלכם כקידומת. **/
    id: string
    /** טקסט תגית ה-@mention הסופי, כולל סימן ה-@. **/
    tag: string
    /** טקסט תגית ה-@mention המקורי, כולל סימן ה-@. **/
    rawTag: string
    /** איזה סוג משתמש תויג. user = חשבון FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** אם המשתמש בחר לצאת מההתראות, זה עדיין יוגדר כ-true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

כאשר hashtags בשימוש ומפורסרים בהצלחה, המידע נשמר ברשימה בשם `hashTags`. לכל אובייקט ברשימה זו
יש את המבנה הבא. ניתן גם להוסיף hashtags ידנית למערך ה-`hashTags` של התגובה לצורך שאילתות, אם `retain` מוגדר.

[inline-code-attrs-start title = 'אובייקט ה-HashTag של התגובה'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** מזהה ה-hashtag. **/
    id: string
    /** טקסט תגית ה-#hashtag הסופי, כולל סימן ה-#. **/
    tag: string
    /** אם ה-hashtag משויך ל-URL מותאם אישית, זה יוגדר. **/
    url?: string
    /** אם עלינו לשמר את ה-hashtag, גם אם הוא אינו קיים בטקסט התגובה, כאשר התגובה מתעדכנת. שימושי לתיוג תגובות מבלי לשנות את טקסט התגובה. **/
    retain?: boolean
}
[inline-code-end]