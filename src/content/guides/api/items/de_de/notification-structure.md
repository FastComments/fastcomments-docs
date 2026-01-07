Ein `Notification`-Objekt repräsentiert eine Benachrichtigung für einen Benutzer.

`Notification`-Objekte werden automatisch erstellt und können nicht über die API erstellt werden. Sie laufen auch nach einem Jahr ab.
Benachrichtigungen können nicht gelöscht werden. Sie können jedoch aktualisiert werden, um `viewed` auf `false` zu setzen, und Sie können nach `viewed` abfragen.

Ein Benutzer kann sich auch von Benachrichtigungen für einen bestimmten Kommentar abmelden, indem er `optedOut` in der Benachrichtigung auf `true` setzt. Sie können sich wieder anmelden, indem Sie es auf `false` setzen.

Es gibt verschiedene Benachrichtigungstypen - prüfen Sie `relatedObjectType` und `type`.

Die Art und Weise, wie Benachrichtigungen erstellt werden, ist recht flexibel und kann durch viele Szenarien ausgelöst werden (siehe `NotificationType`).

Stand heute bedeutet die Existenz einer `Notification` nicht tatsächlich, dass eine E-Mail gesendet wird oder werden sollte. Vielmehr werden die Benachrichtigungen
für den Benachrichtigungs-Feed und verwandte Integrationen verwendet.

Die Struktur des `Notification`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Benachrichtigung Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
