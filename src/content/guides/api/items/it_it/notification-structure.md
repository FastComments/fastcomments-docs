Un oggetto `Notification` rappresenta una notifica per un utente.

Gli oggetti `Notification` vengono creati automaticamente e non possono essere creati tramite l'API. Scadono inoltre dopo un anno.
Le notifiche non possono essere eliminate. Possono comunque essere aggiornate impostando `viewed` su `false`, e puoi interrogarle tramite `viewed`.

Un utente può anche rinunciare alle notifiche per un commento specifico impostando `optedOut` nella notifica su `true`. Puoi riattivare le notifiche impostandolo su `false`.

Esistono diversi tipi di notifiche - controlla `relatedObjectType` e `type`.

I metodi con cui vengono create le notifiche sono piuttosto flessibili e possono essere attivati da molti scenari (vedi `NotificationType`).

Al momento, l'esistenza di una `Notification` non implica necessariamente che una email sia stata o debba essere inviata. Piuttosto, le notifiche
sono utilizzate per il feed delle notifiche e le integrazioni correlate.

La struttura dell'oggetto `Notification` è la seguente:

[inline-code-attrs-start title = 'Struttura dell’oggetto Notification'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Se qualcuno ti ha risposto. **/
    RepliedToMe = 0,
    /** Se qualcuno ha risposto da qualsiasi punto di un thread (anche figli di figli) di un thread su cui hai commentato. **/
    RepliedTransientChild = 1,
    /** Se il tuo commento è stato votato positivamente. **/
    VotedMyComment = 2,
    /** Se viene lasciato un nuovo commento nella radice di una pagina alla quale sei iscritto. **/
    SubscriptionReplyRoot = 3,
    /** Se qualcuno ha commentato il tuo profilo. **/
    CommentedOnProfile = 4,
    /** Se hai un messaggio diretto (DM). **/
    DirectMessage = 5,
    /** TrialLimits è solo per utenti tenant. **/
    TrialLimits = 6,
    /** Se sei stato menzionato con @. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Con SSO, l'id utente ha il formato `<tenant id>:<user id>`. **/
    userId?: string
    /** Quando si lavora con SSO, devi preoccuparti solo di `userId`. **/
    anonUserId?: string
    /** urlId è quasi sempre definito. È opzionale solo per le notifiche a livello di tenant, che sono poco frequenti. **/
    urlId?: string
    /** L'URL è memorizzato nella cache per una navigazione rapida alla fonte della notifica. **/
    url?: string
    /** Il titolo della pagina è memorizzato nella cache per una lettura rapida della fonte della notifica. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Ad esempio, l'id del commento. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // stringa di data
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName e fromUserAvatarSrc sono memorizzati qui per la visualizzazione rapida della notifica. Vengono aggiornati quando l'utente viene aggiornato. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Imposta questo su true per smettere di ricevere notifiche per questo oggetto. **/
    optedOut?: boolean
}
[inline-code-end]

---