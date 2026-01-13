Un objeto `Notification` representa una notificación para un usuario.

Los objetos `Notification` se crean automáticamente y no pueden crearse vía la API. También expiran después de un año.
Las notificaciones no pueden eliminarse. Sin embargo, pueden actualizarse para establecer `viewed` a `false`, y puede consultar por `viewed`.

Un usuario también puede optar por no recibir notificaciones para un comentario específico estableciendo `optedOut` en la notificación a `true`. Puede optar por participar nuevamente estableciéndolo a `false`.

Hay diferentes tipos de notificaciones - consulte `relatedObjectType` y `type`.

Las formas en que se crean las notificaciones es bastante flexible y puede ser activada por muchos escenarios (ver `NotificationType`).

A día de hoy, la existencia de una `Notification` no implica realmente que un email se envíe o deba enviarse. Más bien, las notificaciones
se usan para el feed de notificaciones e integraciones relacionadas.

La estructura del objeto `Notification` es la siguiente:

[inline-code-attrs-start title = 'Estructura de Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
