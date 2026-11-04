Bir `Notification` nesnesi, bir kullanıcı için bir bildirimi temsil eder.

`Notification` nesneleri otomatik olarak oluşturulur ve API aracılığıyla oluşturulamaz. Ayrıca bir yıl sonra sona ererler.
Bildirimler silinemez. Ancak `viewed` değerini `false` olarak ayarlamak için güncellenebilirler ve `viewed` alanına göre sorgulama yapabilirsiniz.

Bir kullanıcı, bildirimdeki `optedOut` alanını `true` olarak ayarlayarak belirli bir yorum için bildirimlerden çıkabilir. `false` olarak ayarlayarak yeniden abone olabilirsiniz.

Farklı bildirim türleri vardır - `relatedObjectType` ve `type` alanlarına bakın.

Bildirimlerin oluşturulma şekilleri oldukça esnektir ve birçok senaryo tarafından tetiklenebilir (bkz. `NotificationType`).

Bugün itibarıyla, bir `Notification` nesnesinin varlığı, bir e-postanın gönderildiği veya gönderilmesi gerektiği anlamına gelmez. Bunun yerine, bildirimler bildirim akışı ve ilgili entegrasyonlar için kullanılır.

`Notification` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'Notification Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Birisi size yanıt verdiyse. **/
    RepliedToMe = 0,
    /** Birisi, yorum yaptığınız bir iş parçacığının (çocukların çocukları dahil) herhangi bir yerinde yanıt verdiyse. **/
    RepliedTransientChild = 1,
    /** Yorumunuz beğenildiyse (up-vote). **/
    VotedMyComment = 2,
    /** Abone olduğunuz bir sayfanın köküne yeni bir yorum bırakıldıysa. **/
    SubscriptionReplyRoot = 3,
    /** Birisi profilinize yorum yaptıysa. **/
    CommentedOnProfile = 4,
    /** Bir DM'niz varsa. **/
    DirectMessage = 5,
    /** TrialLimits yalnızca tenant kullanıcıları içindir. **/
    TrialLimits = 6,
    /** @mention ile bahsedildiySEN. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** SSO ile kullanıcı kimliği `<tenant id>:<user id>` biçimindedir. **/
    userId?: string
    /** SSO ile çalışırken yalnızca `userId` ile ilgilenmeniz gerekir. **/
    anonUserId?: string
    /** urlId neredeyse her zaman tanımlıdır. Yalnızca seyrek görülen tenant düzeyindeki bildirimler için isteğe bağlıdır. **/
    urlId?: string
    /** URL, bildirimin kaynağına hızlı gezinme için önbelleğe alınır. **/
    url?: string
    /** Sayfa Başlığı, bildirim kaynağının hızlı okunması için önbelleğe alınır. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Örneğin, yorum kimliği. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // date string
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName ve fromUserAvatarSrc, bildirimin hızlı görüntülenmesi için burada önbelleğe alınır. Kullanıcı güncellendiğinde güncellenirler. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Bu nesne için bildirim almayı durdurmak için bunu true olarak ayarlayın. **/
    optedOut?: boolean
}
[inline-code-end]