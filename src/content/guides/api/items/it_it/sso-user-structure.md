FastComments fornisce una soluzione SSO facile da usare. Aggiornare le informazioni di un utente con l'integrazione basata su HMAC è semplice come fare in modo che l'utente carichi la pagina con un payload aggiornato.

Tuttavia, potrebbe essere desiderabile gestire un utente al di fuori di quel flusso, per migliorare la coerenza della tua applicazione.

L'SSO User API fornisce un modo per creare, leggere, aggiornare e cancellare oggetti che chiamiamo SSOUsers. Questi oggetti sono diversi dagli Users regolari e vengono mantenuti separati per sicurezza di tipo.

La struttura dell'oggetto SSOUser è la seguente:

[inline-code-attrs-start title = 'Struttura SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // Permesso amministratore - gli SSO users con questo flag vengono fatturati come SSO Admins (separati dagli SSO users regolari)
    isAdminAdmin?: boolean // Permesso amministratore - gli SSO users con questo flag vengono fatturati come SSO Admins (separati dagli SSO users regolari)
    isCommentModeratorAdmin?: boolean // Permesso moderatore - gli SSO users con questo flag vengono fatturati come SSO Moderators (separati dagli SSO users regolari)
    /** Se null, Access Control non verrà applicato all'utente. Se una lista vuota, questo utente non potrà vedere alcuna pagina né usare @mention per altri utenti. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Non permettere ad altri utenti di vedere l'attività di questo utente, inclusi i commenti, nel loro profilo. Il valore predefinito è true per fornire profili sicuri di default. **/
    isProfileActivityPrivate?: boolean
    /** Non permettere ad altri utenti di lasciare commenti sul profilo dell'utente, o vedere i commenti esistenti del profilo. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Non permettere ad altri utenti di inviare messaggi diretti a questo utente. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Configurazione opzionale per i badge utente. **/
    badgeConfig?: {
        /** Array di ID dei badge da assegnare all'utente. Limitato a 30 badge. L'ordine è rispettato. **/
        badgeIds: string[]
        /** Se true, sostituisce tutti i badge esistenti mostrati con quelli forniti. Se false, aggiunge ai badge esistenti. **/
        override?: boolean
        /** Se true, aggiorna le proprietà di visualizzazione dei badge dalla configurazione del tenant. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

Gli SSO users vengono fatturati in modo diverso in base ai loro flag di permesso:

- **Regular SSO Users**: Utenti senza permessi di admin o moderatore vengono fatturati come regular SSO users
- **SSO Admins**: Utenti con i flag `isAccountOwner` o `isAdminAdmin` vengono fatturati separatamente come SSO Admins (stessa tariffa degli admin regolari del tenant)
- **SSO Moderators**: Utenti con il flag `isCommentModeratorAdmin` vengono fatturati separatamente come SSO Moderators (stessa tariffa dei moderatori regolari)

**Importante**: Per evitare doppia fatturazione, il sistema deduplica automaticamente gli SSO users rispetto agli utenti e moderatori regolari del tenant basandosi sull'indirizzo email. Se un SSO user ha la stessa email di un utente o moderatore regolare del tenant, non verrà fatturato due volte.

### Access Control

Gli utenti possono essere suddivisi in gruppi. Questo è lo scopo del campo `groupIds`, ed è opzionale.

### @Mentions

Di default `@mentions` utilizzerà `username` per cercare altri sso users quando viene digitato il carattere `@`. Se viene utilizzato `displayName`, allora i risultati che corrispondono a `username` saranno ignorati quando c'è una corrispondenza per `displayName`, e i risultati della ricerca per gli `@mention` useranno `displayName`.

### Subscriptions

Con FastComments, gli utenti possono iscriversi a una pagina cliccando l'icona della campanella nel widget dei commenti e selezionando Subscribe.

Con un utente regolare, inviamo loro email di notifica in base alle loro impostazioni di notifica.

Con gli SSO Users, abbiamo separato questo comportamento per compatibilità retroattiva. Gli utenti riceveranno queste email di notifica aggiuntive per le subscription solo se imposti `optedInSubscriptionNotifications` su `true`.

### Badges

Puoi assegnare badge agli SSO users usando la proprietà `badgeConfig`. I badge sono indicatori visivi che appaiono accanto al nome di un utente nei commenti.

- `badgeIds` - Un array di ID dei badge da assegnare all'utente. Devono essere ID validi di badge creati nel tuo account FastComments. Limitato a 30 badge.
- `override` - Se true, tutti i badge esistenti visualizzati sui commenti saranno sostituiti con quelli forniti. Se false o omesso, i badge forniti saranno aggiunti a quelli esistenti.
- `update` - Se true, le proprietà di visualizzazione dei badge verranno aggiornate dalla configurazione del tenant ogni volta che l'utente effettua il login.