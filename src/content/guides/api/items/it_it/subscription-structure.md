Un oggetto `Subscription` rappresenta una sottoscrizione per un utente.

Gli oggetti `Subscription` vengono creati quando un utente clicca la campanella delle notifiche nel widget dei commenti e seleziona "Iscriviti a questa pagina".

Le sottoscrizioni possono anche essere create via API.

Avere un oggetto `Subscription` fa sì che vengano generati oggetti `Notification` e inviate email quando vengono lasciati nuovi commenti nella radice della pagina associata a cui la `Subscription` si riferisce. L'invio delle email dipende dal tipo di utente. Per gli utenti normali ciò dipende da `optedInNotifications`. Per gli utenti SSO ciò dipende da `optedInSubscriptionNotifications`. Nota che alcune applicazioni potrebbero non avere il concetto di una pagina accessibile via web, nel qual caso impostare semplicemente `urlId` sull'id dell'elemento a cui si sta sottoscrivendo (stesso valore per `urlId` che si passerebbe al widget dei commenti).

La struttura dell'oggetto `Subscription` è la seguente:

[inline-code-attrs-start title = 'Struttura di Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** Con SSO, l'id utente ha il formato `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // stringa della data
}
[inline-code-end]

---