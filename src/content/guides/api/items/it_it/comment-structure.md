Un oggetto `Comment` rappresenta un commento lasciato da un utente.

La relazione tra commenti genitore e figli è definita tramite `parentId`.

La struttura dell'oggetto Comment è la seguente:

[inline-code-attrs-start title = 'Struttura del Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SOLO LETTURA: Impostato su true se il motore antispam ha determinato che il commento era spam. **/
    aiDeterminedSpam?: boolean
    /** Indica se il commento è approvato per la visualizzazione. Impostato su true quando si salva il commento, altrimenti sarà nascosto. **/
    approved?: boolean
    /** L'avatar dell'utente. **/
    avatarSrc?: string
    /** Commenti figli. Non popolato in tutti gli scenari. Usato quando asTree è impostato su true tramite l'API. **/
    children: Comment[]
    /** Il commento grezzo dell'utente. **/
    comment: string
    /** SOLO LETTURA: Il commento dell'utente parsato in HTML. **/
    commentHTML?: string
    /** L'email dell'autore del commento. Richiesta se i commenti anonimi sono disabilitati. **/
    commenterEmail?: string
    /** Il link dell'autore del commento (per esempio, il loro blog). **/
    commenterLink?: string
    /** Il nome dell'autore del commento. Sempre richiesto. Se non disponibile, impostare qualcosa come "Anonimo". **/
    commenterName: string
    /** La data in cui è stato inserito il commento, in epoch UTC. **/
    date: number
    /** L'"etichetta di visualizzazione" per il commento - per esempio "Admin", "Moderator", o qualcosa come "VIP User". **/
    displayLabel?: string
    /** Il dominio su cui è stato pubblicato il commento. **/
    domain?: string
    /** SOLO LETTURA: Il numero di volte in cui il commento è stato segnalato. **/
    flagCount?: number
    /** Gli #hashtag scritti nel commento che sono stati correttamente parsati. È anche possibile aggiungere manualmente hashtag per le query, ma non verranno visualizzati automaticamente nel testo del commento. **/
    hashTags?: CommentHashTag[]
    /** SOLO LETTURA: Il commento contiene immagini? **/
    hasImages?: boolean
    /** SOLO LETTURA: Il commento contiene link? **/
    hasLinks?: boolean
    /** SOLO LETTURA: L'id univoco del commento. **/
    id: string
    /** Solo alla creazione! Viene hashato per l'archiviazione. **/
    ip?: string
    /** SOLO LETTURA: L'utente corrente ha bloccato l'autore di questo commento? **/
    isBlocked?: boolean
    /** SOLO LETTURA: Il commento è stato scritto da un admin? Impostato automaticamente in base a userId. **/
    isByAdmin?: boolean
    /** SOLO LETTURA: Il commento è stato scritto da un moderatore? Impostato automaticamente in base a userId. **/
    isByModerator?: boolean
    /** Impostato su true se il commento è stato eliminato "soft" (è stato lasciato un segnaposto a causa di qualche altra configurazione). **/
    isDeleted?: boolean
    /** Impostato su true se l'account dell'utente è stato eliminato e il commento è stato mantenuto. **/
    isDeletedUser?: boolean
    /** SOLO LETTURA: È stato segnalato dall'utente attualmente connesso (contextUserId)? **/
    isFlagged?: boolean
    /** Il commento è fissato in alto (pinned)? **/
    isPinned?: boolean
    /** Il commento è bloccato per nuove risposte (i moderatori possono ancora rispondere)? **/
    isLocked?: boolean
    /** Il commento è spam? **/
    isSpam?: boolean
    /** SOLO LETTURA: Il commento è stato votato negativamente dall'utente corrente (contextUserId)? **/
    isVotedDown?: boolean
    /** SOLO LETTURA: Il commento è stato votato positivamente dall'utente corrente (contextUserId)? **/
    isVotedUp?: boolean
    /** La locale del commento. Se non fornita, verrà ricavata dall'header HTTP Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SOLO LETTURA: Le @mention scritte nel commento che sono state correttamente analizzate. **/
    mentions?: CommentUserMention[]
    /** Metadati opzionali associati al commento. **/
    meta?: Record<string, string | number | boolean>
    /** L'elenco opzionale degli id dei gruppi di moderazione associati a questo commento. **/
    moderationGroupIds?: string[]|null
    /** SOLO LETTURA: L'id dell'oggetto voto che corrisponde al voto dell'utente corrente (contextUserId) su questo commento. **/
    myVoteId?: string
    /** Indica se sono state inviate notifiche per questo commento agli autori. Per evitare l'invio di notifiche durante le importazioni, impostare su true. **/
    notificationSentForParent?: boolean
    /** Indica se sono state inviate notifiche per questo commento agli utenti del tenant. Per evitare l'invio di notifiche durante le importazioni, impostare su true. **/
    notificationSentForParentTenant?: boolean
    /** Il titolo della pagina su cui era presente questo commento. **/
    pageTitle?: string
    /** Se stiamo rispondendo a un commento, questo è l'ID a cui stiamo rispondendo. **/
    parentId?: string|null
    /** Indica se il commento è contrassegnato come revisionato. **/
    reviewed: boolean
    /** L'id del tenant a cui appartiene il commento. **/
    tenantId: string
    /** L'utente che ha scritto il commento. Creato automaticamente quando si salva un commento con nome/email. **/
    userId?: string|null
    /** L'URL della posizione in cui questo commento è visibile, come un post del blog. **/
    url: string
    /** Una versione "pulita" dello urlId che ci hai fornito. Durante il salvataggio specifichi questo campo, ma quando recuperi il commento questo verrà "pulito" e il valore originale spostato in "urlIdRaw". **/
    urlId: string
    /** SOLO LETTURA: Lo urlId originale che ci hai fornito. **/
    urlIdRaw?: string
    /** L'utente e questo commento sono verificati? **/
    verified: boolean
    /** Numero di voti positivi. **/
    votesUp?: number
    /** Numero di voti negativi. **/
    votesDown?: number
    /** Il "karma" del commento (= voti positivi - voti negativi). **/
    votes?: number
}
[inline-code-end]

Alcuni di questi campi sono contrassegnati come `READONLY` - vengono restituiti dall'API ma non possono essere impostati.

### Struttura del Testo del Commento

I commenti sono scritti in una variante di markdown di FastComments, che è semplicemente markdown più i tradizionali tag in stile `bbcode` per le immagini, ad esempio `[img]path[/img]`.

Il testo è memorizzato in due campi. Il testo inserito dall'utente è memorizzato non modificato nel campo `comment`. Questo viene renderizzato e memorizzato nel campo `commentHTML`.

I tag HTML consentiti sono `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Si consiglia di renderizzare l'HTML, dato che è un sottinsieme molto piccolo di HTML: creare un renderer è piuttosto semplice. Esistono diverse librerie per React Native e Flutter, ad esempio, che possono aiutare in questo

È possibile scegliere di renderizzare il valore non normalizzato del campo `comment`. [Un parser di esempio è qui.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Il parser di esempio può anche essere adattato per funzionare con HTML e trasformare i tag HTML negli elementi attesi da renderizzare per la tua piattaforma. 

### Menzioni

Quando gli utenti vengono taggati in un commento, le informazioni vengono memorizzate in una lista chiamata `mentions`. Ogni oggetto in quella lista
ha la seguente struttura.

[inline-code-attrs-start title = 'Oggetto Mention Utente del Commento'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id utente. Per gli utenti SSO, questo avrà il prefisso dell'id del tuo tenant. **/
    id: string
    /** Il testo finale del tag @mention, incluso il simbolo @. **/
    tag: string
    /** Il testo originale del tag @mention, incluso il simbolo @. **/
    rawTag: string
    /** Il tipo di utente taggato. user = account FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se l'utente rinuncia alle notifiche, questo sarà comunque impostato su true. **/
    sent: boolean
}
[inline-code-end]

### Hashtag

Quando gli hashtag vengono usati e correttamente parsati, le informazioni sono memorizzate in una lista chiamata `hashTags`. Ogni oggetto in quella lista
ha la seguente struttura. Gli hashtag possono anche essere aggiunti manualmente all'array `hashTags` del commento per le query, se `retain` è impostato.

[inline-code-attrs-start title = 'Oggetto Hashtag del Commento'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** L'id dell'hashtag. **/
    id: string
    /** Il testo finale del tag #hashtag, incluso il simbolo #. **/
    tag: string
    /** Se l'hashtag è associato a un URL personalizzato, questo sarà definito. **/
    url?: string
    /** Se dobbiamo conservare l'hashtag, anche se non esiste nel testo del commento, quando il commento viene aggiornato. Utile per taggare i commenti senza modificare il testo del commento. **/
    retain?: boolean
}
[inline-code-end]

---