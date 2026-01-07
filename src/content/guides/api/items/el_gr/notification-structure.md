Ένα αντικείμενο `Notification` αντιπροσωπεύει μια ειδοποίηση για έναν χρήστη.

Τα αντικείμενα `Notification` δημιουργούνται αυτόματα και δεν μπορούν να δημιουργηθούν μέσω του API. Επίσης λήγουν μετά από ένα έτος.
Οι ειδοποιήσεις δεν μπορούν να διαγραφούν. Μπορούν όμως να ενημερωθούν για να ορίσουν το `viewed` σε `false`, και μπορείτε να κάνετε ερώτημα με βάση το `viewed`.

Ένας χρήστης μπορεί επίσης να εξαιρεθεί από τις ειδοποιήσεις για ένα συγκεκριμένο σχόλιο ορίζοντας το `optedOut` στην ειδοποίηση σε `true`. Μπορείτε να συμμετάσχετε ξανά ορίζοντάς το σε `false`.

Υπάρχουν διαφορετικοί τύποι ειδοποιήσεων - ελέγξτε τα `relatedObjectType` και `type`.

Οι τρόποι δημιουργίας ειδοποιήσεων είναι αρκετά ευέλικτοι και μπορούν να ενεργοποιηθούν από πολλά σενάρια (δείτε `NotificationType`).

Μέχρι σήμερα, η ύπαρξη μιας `Notification` δεν συνεπάγεται πραγματικά ότι αποστέλλεται ή πρέπει να αποσταλεί email. Αντίθετα, οι ειδοποιήσεις
χρησιμοποιούνται για τη ροή ειδοποιήσεων και τις σχετικές ενσωματώσεις.

Η δομή για το αντικείμενο `Notification` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Ειδοποίησης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
