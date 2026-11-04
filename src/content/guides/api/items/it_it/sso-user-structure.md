FastComments fornisce una soluzione SSO facile da usare. Aggiornare le informazioni di un utente con l'integrazione basata su HMAC è
semplice come far caricare all'utente la pagina con un payload aggiornato.

Tuttavia, può essere desiderabile gestire un utente al di fuori di quel flusso, per migliorare la coerenza della tua applicazione.

La SSO User API fornisce un modo per eseguire operazioni CRUD su oggetti che chiamiamo SSOUsers. Questi oggetti sono diversi dagli Users regolari e
vengono mantenuti separati per type safety.

La struttura dell'oggetto SSOUser è la seguente:

[inline-code-attrs-start title = 'Struttura di SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Permessi di amministratore - gli SSO users con questo flag vengono fatturati come SSO Admins (separati dagli SSO users regolari)
    isAdminAdmin?: boolean // Permessi di amministratore - gli SSO users con questo flag vengono fatturati come SSO Admins (separati dagli SSO users regolari)
    isCommentModeratorAdmin?: boolean // Permessi di moderatore - gli SSO users con questo flag vengono fatturati come SSO Moderators (separati dagli SSO users regolari)
    /** Se null, l'Access Control non verrà applicato all'utente. Se è una lista vuota, questo utente non potrà vedere alcuna pagina né fare @mention ad altri utenti. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Non far vedere ad altri utenti l'attività di questo utente, inclusi i commenti, sul suo profilo. Il valore predefinito è true per fornire profili sicuri di default. **/
    isProfileActivityPrivate?: boolean
    /** Non far sì che altri utenti possano lasciare commenti sul profilo dell'utente, o vedere i commenti esistenti sul profilo. Il valore predefinito è false. **/
    isProfileCommentsPrivate?: boolean
    /** Non far sì che altri utenti possano inviare messaggi diretti a questo utente. Il valore predefinito è false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configurazione opzionale per i badge dell'utente. **/
    badgeConfig?: {
        /** Array di ID badge da assegnare all'utente. Limitato a 30 badge. L'ordine viene rispettato. Questi sono badge globali visibili su tutte le pagine. **/
        badgeIds: string[]
        /** Array di ID badge con ambito limitato alla pagina corrente (urlId). Questi badge vengono mostrati solo sulla pagina a cui sono stati assegnati. **/
        pageBadgeIds?: string[]
        /** Se true, sostituisce tutti i badge attualmente visualizzati con quelli forniti. I badge globali e quelli con ambito di pagina vengono sovrascritti indipendentemente. Se false, aggiunge ai badge esistenti. **/
        override?: boolean
        /** Se true, aggiorna le proprietà di visualizzazione dei badge dalla configurazione del tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Fatturazione degli SSO Users

Gli SSO users vengono fatturati in modo diverso in base ai loro flag di permessi:

- **SSO Users regolari**: gli utenti senza permessi di amministratore o moderatore vengono fatturati come SSO users regolari
- **SSO Admins**: gli utenti con i flag `isAccountOwner` o `isAdminAdmin` vengono fatturati separatamente come SSO Admins (stessa tariffa degli amministratori regolari del tenant)
- **SSO Moderators**: gli utenti con il flag `isCommentModeratorAdmin` vengono fatturati separatamente come SSO Moderators (stessa tariffa dei moderatori regolari)

**Importante**: per prevenire una doppia fatturazione, il sistema deduplica automaticamente gli SSO users rispetto agli utenti e moderatori regolari del tenant tramite l'indirizzo email. Se un SSO user ha la stessa email di un utente o moderatore regolare del tenant, non verrà fatturato due volte.

### Access Control

Gli utenti possono essere suddivisi in gruppi. È a questo che serve il campo `groupIds`, ed è opzionale.

### @Mentions

Per impostazione predefinita, gli `@mentions` utilizzeranno `username` per cercare altri sso users quando viene digitato il carattere `@`. Se viene usato `displayName`, i risultati corrispondenti a `username` verranno ignorati quando c'è una corrispondenza per `displayName`, e i risultati della ricerca `@mention` utilizzeranno `displayName`.

### Sottoscrizioni

Con FastComments, gli utenti possono sottoscrivere una pagina cliccando l'icona a forma di campanella nel widget dei commenti e poi cliccando Subscribe.

Con un utente regolare, inviamo email di notifica in base alle impostazioni di notifica dell'utente.

Con gli SSO Users, separiamo questo comportamento per compatibilità con le versioni precedenti. Gli utenti riceveranno queste email di notifica di sottoscrizione aggiuntive solo se imposti `optedInSubscriptionNotifications` su `true`.

### Badge

Puoi assegnare badge agli SSO users usando la proprietà `badgeConfig`. I badge sono indicatori visivi che appaiono accanto al nome di un utente nei commenti.

- `badgeIds` - Un array di ID badge da assegnare all'utente. Questi sono badge globali visibili su tutte le pagine. Devono essere ID badge validi creati nel tuo account FastComments. Limitato a 30 badge.
- `pageBadgeIds` - Un array opzionale di ID badge con ambito limitato alla pagina corrente (`urlId`). Questi badge vengono mostrati solo sulla pagina a cui sono stati assegnati. Pagine diverse possono avere badge di pagina diversi per lo stesso utente.
- `override` - Se true, tutti i badge attualmente visualizzati verranno sostituiti con quelli forniti. I badge globali e quelli con ambito di pagina vengono sovrascritti indipendentemente — sovrascrivere i badge globali non influisce sui badge con ambito di pagina, e viceversa. Se false o omesso, i badge forniti verranno aggiunti ai badge esistenti.
- `update` - Se true, le proprietà di visualizzazione dei badge verranno aggiornate dalla configurazione del tenant ogni volta che l'utente effettua il login.