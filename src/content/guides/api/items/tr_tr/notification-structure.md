A `Notification` object represents a notification for a user.

`Notification` objects are created automatically and cannot be created via the API. They also expire after one year.
Notifications cannot be deleted. They can however be updated to set `viewed` to `false`, and you can query by `viewed`.

A user may also opt out of notifications for a specific comment by setting `optedOut` in the notification to `true`. You can opt in again by setting it to `false`.

There are different notification types - check `relatedObjectType` and `type`.

The ways notifications are created is quite flexible and can be triggered by many scenarios (see `NotificationType`).

As of today, the existence of a `Notification` does not actually imply an email is or should be sent. Rather, the notifications
are used for the notification feed and related integrations.

The structure for the `Notification` object is as follows:

[inline-code-attrs-start title = 'Bildirim Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Birisi size yanıt verdiğinde. **/
    RepliedToMe = 0,
    /** Birisi, yorum yaptığınız bir dizide (alt diziler dahil) herhangi bir yere yanıt verdiğinde. **/
    RepliedTransientChild = 1,
    /** Yorumunuza oy verildiğinde. **/
    VotedMyComment = 2,
    /** Abone olduğunuz bir sayfanın köküne yeni bir yorum bırakıldığında. **/
    SubscriptionReplyRoot = 3,
    /** Birisi profilinize yorum yaptığında. **/
    CommentedOnProfile = 4,
    /** Size bir DM geldiğinde. **/
    DirectMessage = 5,
    /** TrialLimits yalnızca tenant kullanıcıları için. **/
    TrialLimits = 6,
    /** Eğer size @ ile bahsedildiyse. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** SSO ile, kullanıcı kimliği formatı `<tenant id>:<user id>` şeklindedir. **/
    userId?: string
    /** SSO ile çalışırken yalnızca `userId` ile ilgilenmeniz gerekir. **/
    anonUserId?: string
    /** urlId neredeyse her zaman tanımlıdır. Yalnızca tenant düzeyindeki bildirimler için isteğe bağlıdır; bunlar nadirdir. **/
    urlId?: string
    /** Bildirimin kaynağına hızlı gezinme için URL önbelleğe alınır. **/
    url?: string
    /** Bildirim kaynağının hızlı okunması için sayfa başlığı önbelleğe alınır. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Örneğin, yorum kimliği. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // tarih dizesi
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName ve fromUserAvatarSrc bildirimin hızlı görüntülenmesi için burada önbelleğe alınır. Kullanıcı güncellendiğinde güncellenirler. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Bu nesne için bildirim almamak için bunu true olarak ayarlayın. **/
    optedOut?: boolean
}
[inline-code-end]