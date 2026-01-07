Обектът `Notification` представлява известие за потребител.

Обектите `Notification` се създават автоматично и не могат да бъдат създавани чрез API. Те също изтичат след една година.
Известията не могат да бъдат изтривани. Въпреки това те могат да бъдат актуализирани, за да зададете `viewed` на `false`, и можете да търсите по `viewed`.

Потребителят може също да се откаже от известия за конкретен коментар, като зададе `optedOut` в известието на `true`. Можете да се включите отново, като го зададете на `false`.

Има различни типове известия - проверете `relatedObjectType` и `type`.

Начините, по които се създават известия, са доста гъвкави и могат да бъдат задействани от много сценарии (вижте `NotificationType`).

Към днешна дата съществуването на `Notification` всъщност не означава, че имейл е или трябва да бъде изпратен. По-скоро известията
се използват за емисия от известия и свързани интеграции.

Структурата на обекта `Notification` е следната:

[inline-code-attrs-start title = 'Структура на Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
