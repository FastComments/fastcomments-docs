Об'єкт `Notification` представляє сповіщення для користувача.

Об'єкти `Notification` створюються автоматично і не можуть бути створені через API. Вони також закінчують термін дії через один рік.
Сповіщення не можна видалити. Однак їх можна оновити, встановивши `viewed` в `false`, і можна виконувати запити за `viewed`.

Користувач також може відмовитись від сповіщень для конкретного коментаря, встановивши `optedOut` у сповіщенні в `true`. Ви можете знову підписатися, встановивши його в `false`.

Існують різні типи сповіщень — перевіряйте `relatedObjectType` і `type`.

Способи створення сповіщень досить гнучкі і можуть бути викликані багатьма сценаріями (див. `NotificationType`).

На сьогодні наявність `Notification` фактично не означає, що електронний лист надсилається або має бути надісланий. Швидше за все, сповіщення
використовуються для стрічки сповіщень та пов'язаних інтеграцій.

Структура об'єкта `Notification` виглядає так:

[inline-code-attrs-start title = 'Структура Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Якщо хтось відповів вам. **/
    RepliedToMe = 0,
    /** Якщо хтось відповів будь-де в треді (навіть нащадки нащадків) треда, в якому ви коментували. **/
    RepliedTransientChild = 1,
    /** Якщо за ваш коментар проголосували. **/
    VotedMyComment = 2,
    /** Якщо на корені сторінки, на яку ви підписані, залишено новий коментар. **/
    SubscriptionReplyRoot = 3,
    /** Якщо хтось прокоментував ваш профіль. **/
    CommentedOnProfile = 4,
    /** Якщо у вас є приватне повідомлення (DM). **/
    DirectMessage = 5,
    /** TrialLimits призначено лише для користувачів tenant. **/
    TrialLimits = 6,
    /** Якщо вас згадали за допомогою @. **/
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
    createdAt: string // рядок дати
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

---