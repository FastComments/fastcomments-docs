אובייקט `Notification` מייצג התראה עבור משתמש.

אובייקטי `Notification` נוצרים אוטומטית ולא ניתן ליצור אותם דרך ה-API. הם גם פגים לאחר שנה אחת.
לא ניתן למחוק התראות. עם זאת, ניתן לעדכן אותן כדי להגדיר את `viewed` ל-`false`, וניתן לבצע שאילתה לפי `viewed`.

משתמש יכול גם לבחור לצאת מהתראות עבור תגובה ספציפית על ידי הגדרת `optedOut` בהתראה ל-`true`. ניתן להצטרף מחדש על ידי הגדרתו ל-`false`.

ישנם סוגי התראות שונים - בדוק את `relatedObjectType` ו-`type`.

הדרכים ליצירת התראות הן די גמישות וניתנות להפעלה על ידי תרחישים רבים (ראה `NotificationType`).

נכון להיום, קיום `Notification` לא באמת מרמז שאימייל נשלח או צריך להישלח. במקום זאת, ההתראות
משמשות לפיד ההתראות ואינטגרציות קשורות.

המבנה עבור אובייקט `Notification` הוא כדלקמן:

[inline-code-attrs-start title = 'מבנה התראה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** If someone replied to you. **/
    RepliedToMe = 0,
    /** If someone replied anywhere in a thread (even children of children) of a thread you commented on. **/
    RepliedTransientChild = 1,
    /** If your comment was up-voted. **/
    VotedMyComment = 2,
    /** If a new comment is left on the root of a page you're subscribed to. **/
    SubscriptionReplyRoot = 3,
    /** If someone commented on your profile. **/
    CommentedOnProfile = 4,
    /** If you have a DM. **/
    DirectMessage = 5,
    /** TrialLimits is for tenant users only. **/
    TrialLimits = 6,
    /** If you were @mentioned. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId?: string
    /** When working with SSO, you only have to worry about `userId`. **/
    anonUserId?: string
    /** urlId is almost always defined. It is only optional for tenant-level notifications, which are infrequent. **/
    urlId?: string
    /** URL is cached for quick navigation to the source of the notification. **/
    url?: string
    /** Page Title is cached for quick reading of notification source. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** For example, comment id. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName and fromUserAvatarSrc are cached here for quick displaying of the notification. They are updated when the user is updated. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Set this to true to stop getting notifications for this object. **/
    optedOut?: boolean
}
[inline-code-end]
