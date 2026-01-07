Un objet `Notification` représente une notification pour un utilisateur.

Les objets `Notification` sont créés automatiquement et ne peuvent pas être créés via l'API. Ils expirent également après un an.
Les notifications ne peuvent pas être supprimées. Elles peuvent cependant être mises à jour pour définir `viewed` à `false`, et vous pouvez filtrer par `viewed`.

Un utilisateur peut également se désabonner des notifications pour un commentaire spécifique en définissant `optedOut` dans la notification à `true`. Vous pouvez vous réabonner en le définissant à `false`.

Il existe différents types de notifications - vérifiez `relatedObjectType` et `type`.

Les façons dont les notifications sont créées sont assez flexibles et peuvent être déclenchées par de nombreux scénarios (voir `NotificationType`).

À ce jour, l'existence d'une `Notification` n'implique pas réellement qu'un email est ou devrait être envoyé. Plutôt, les notifications
sont utilisées pour le fil de notifications et les intégrations connexes.

La structure de l'objet `Notification` est la suivante :

[inline-code-attrs-start title = 'Structure de Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
