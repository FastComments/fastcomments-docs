Objekt `Notification` predstavlja obvestilo za uporabnika.

Objekti `Notification` se ustvarijo samodejno in jih ni mogoče ustvariti prek API-ja. Prav tako potečejo po enem letu.
Obvestil ni mogoče izbrisati. Lahko pa se posodobijo, tako da nastavite `viewed` na `false`, in lahko poizvedujete po `viewed`.

Uporabnik se lahko tudi odjavi od obvestil za določen komentar tako, da v obvestilu nastavi `optedOut` na `true`. Ponovno se lahko prijavite tako, da nastavite `optedOut` na `false`.

Obstajajo različne vrste obvestil - preverite `relatedObjectType` in `type`.

Načini ustvarjanja obvestil so precej prilagodljivi in jih lahko sprožijo številni scenariji (glejte `NotificationType`).

Trenutno prisotnost `Notification` ne pomeni nujno, da je bilo ali bi moralo biti poslano e-poštno sporočilo. Namesto tega se obvestila uporabljajo za feed obvestil in sorodne integracije.

Struktura za objekt `Notification` je naslednja:

[inline-code-attrs-start title = 'Struktura obvestila'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Če vam je nekdo odgovoril. **/
    RepliedToMe = 0,
    /** Če je nekdo odgovoril kjerkoli v niti (tudi otroci otrok) niti, kjer ste komentirali. **/
    RepliedTransientChild = 1,
    /** Če je bil vaš komentar podprt z glasom gor (up-voted). **/
    VotedMyComment = 2,
    /** Če je na korenu strani, na katero ste naročeni, objavljen nov komentar. **/
    SubscriptionReplyRoot = 3,
    /** Če je nekdo komentiral vaš profil. **/
    CommentedOnProfile = 4,
    /** Če imate zasebno sporočilo (DM). **/
    DirectMessage = 5,
    /** TrialLimits je namenjen samo uporabnikom najemnika. **/
    TrialLimits = 6,
    /** Če ste bili omenjeni z @. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** Pri SSO je uporabniški id v formatu `<tenant id>:<user id>`. **/
    userId?: string
    /** Pri delu s SSO vas zanima samo `userId`. **/
    anonUserId?: string
    /** urlId je skoraj vedno določen. To je opcijsko samo za obvestila na ravni najemnika, ki so redka. **/
    urlId?: string
    /** URL je predpomnjen za hitro navigacijo do vira obvestila. **/
    url?: string
    /** Naslov strani je predpomnjen za hitro branje vira obvestila. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Na primer, id komentarja. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // niz datuma
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName and fromUserAvatarSrc so tukaj predpomnjena za hitro prikazovanje obvestila. Posodobita se, ko se uporabnik posodobi. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Nastavite to na true, da prenehate prejemati obvestila za ta objekt. **/
    optedOut?: boolean
}
[inline-code-end]

---