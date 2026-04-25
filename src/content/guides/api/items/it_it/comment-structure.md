Un oggetto `Comment` rappresenta un commento lasciato da un utente.

La relazione tra commenti padre e figli è definita tramite `parentId`.

La struttura dell'oggetto Comment è la seguente:

[inline-code-attrs-start title = 'Struttura del Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Impostato su true se il motore anti-spam ha determinato che il commento era spam. **/
    aiDeterminedSpam?: boolean
    /** Se il commento è approvato per la visualizzazione. Impostare su true quando si salva il commento, altrimenti sarà nascosto. **/
    approved?: boolean
    /** L'avatar dell'utente. **/
    avatarSrc?: string
    /** Commenti figli. Non popolato in tutti gli scenari. Utilizzato quando asTree è impostato su true tramite l'API. **/
    children: Comment[]
    /** Il commento grezzo dell'utente. **/
    comment: string
    /** READONLY: Il commento dell'utente analizzato in HTML. **/
    commentHTML?: string
    /** L'email del commentatore. Richiesta se i commenti anonimi sono disattivati. **/
    commenterEmail?: string
    /** Il link del commentatore (per esempio, il loro blog). **/
    commenterLink?: string
    /** Il nome del commentatore. Sempre richiesto. Se non disponibile, impostare qualcosa come "Anonimo". **/
    commenterName: string
    /** La data in cui è stato lasciato il commento, in epoch UTC. **/
    date: number
    /** L'"etichetta di visualizzazione" per il commento - per esempio "Admin", "Moderator" o qualcosa come "VIP User". **/
    displayLabel?: string
    /** Il dominio su cui è stato pubblicato il commento. **/
    domain?: string
    /** READONLY: Il numero di volte che il commento è stato segnalato. **/
    flagCount?: number
    /** Gli #hashtag scritti nel commento che sono stati analizzati correttamente. È inoltre possibile aggiungere manualmente hashtag, per le query, ma non verranno mostrati automaticamente nel testo del commento. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Il commento contiene immagini? **/
    hasImages?: boolean
    /** READONLY: Il commento contiene link? **/
    hasLinks?: boolean
    /** READONLY: L'id univoco del commento. **/
    id: string
    /** Solo alla creazione! Viene hashato per l'archiviazione. **/
    ip?: string
    /** READONLY: L'utente corrente ha bloccato l'autore di questo commento? **/
    isBlocked?: boolean
    /** READONLY: Il commento è di un admin? Impostato automaticamente in base a userId. **/
    isByAdmin?: boolean
    /** READONLY: Il commento è di un moderatore? Impostato automaticamente in base a userId. **/
    isByModerator?: boolean
    /** Impostato su true se il commento è stato eliminato soft (deve essere lasciato un segnaposto a causa di qualche altra configurazione). **/
    isDeleted?: boolean
    /** Impostato su true se l'account dell'utente è stato eliminato e il commento doveva essere mantenuto. **/
    isDeletedUser?: boolean
    /** READONLY: È stato segnalato dall'utente attualmente connesso (contextUserId)? **/
    isFlagged?: boolean
    /** Il commento è appuntato (pinned)? **/
    isPinned?: boolean
    /** Il commento è bloccato? Quando true, nessuno (inclusi i moderatori) può rispondere, modificare o eliminarlo fino a quando non viene sbloccato. **/
    isLocked?: boolean
    /** Il commento è spam? **/
    isSpam?: boolean
    /** READONLY: Il commento è stato votato negativamente dall'utente corrente (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Il commento è stato votato positivamente dall'utente corrente (contextUserId)? **/
    isVotedUp?: boolean
    /** La locale del commento. Se non fornita, verrà derivata dall'header HTTP Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: Le @menzioni scritte nel commento che sono state analizzate correttamente. **/
    mentions?: CommentUserMention[]
    /** Metadati opzionali associati al commento. **/
    meta?: Record<string, string | number | boolean>
    /** L'elenco opzionale di id dei gruppi di moderazione associati a questo commento. **/
    moderationGroupIds?: string[]|null
    /** READONLY: L'id dell'oggetto voto che corrisponde al voto dell'utente corrente (contextUserId) su questo commento. **/
    myVoteId?: string
    /** Se sono state inviate notifiche per questo commento ai commentatori. Per evitare l'invio di notifiche durante le importazioni, impostare questo valore su true. **/
    notificationSentForParent?: boolean
    /** Se sono state inviate notifiche per questo commento agli utenti del tenant. Per evitare l'invio di notifiche durante le importazioni, impostare questo valore su true. **/
    notificationSentForParentTenant?: boolean
    /** Il titolo della pagina su cui si trovava questo commento. **/
    pageTitle?: string
    /** Se stiamo rispondendo a un commento, questo è l'ID a cui stiamo rispondendo. **/
    parentId?: string|null
    /** Se il commento è contrassegnato come revisionato. **/
    reviewed: boolean
    /** L'id del tenant a cui appartiene il commento. **/
    tenantId: string
    /** L'utente che ha scritto il commento. Creato automaticamente quando si salva un commento con nome/email. **/
    userId?: string|null
    /** L'URL della posizione in cui questo commento è visibile, come un post del blog. **/
    url: string
    /** Una versione "pulita" dell'urlId che ci hai fornito. Durante il salvataggio, specifichi questo campo, ma quando recuperi il commento questo verrà "ripulito" e il tuo valore originale spostato in "urlIdRaw". **/
    urlId: string
    /** READONLY: L'urlId originale che ci hai passato. **/
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

Alcuni di questi campi sono contrassegnati come `READONLY` - questi vengono restituiti dall'API ma non possono essere impostati.

### Struttura del Testo del Commento

I commenti sono scritti in una variante di markdown di FastComments, che è semplicemente markdown più i tradizionali tag in stile `bbcode` per le immagini, come `[img]path[/img]`.

Il testo è memorizzato in due campi. Il testo inserito dall'utente è memorizzato non modificato nel campo `comment`. Questo viene renderizzato e memorizzato nel campo `commentHTML`.

I tag HTML consentiti sono `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Si raccomanda di renderizzare l'HTML, dato che è un sottoinsieme molto piccolo di HTML, costruire un renderer è piuttosto semplice. Esistono diverse librerie per React Native e Flutter, per esempio, che possono aiutare in questo

Puoi scegliere di renderizzare il valore non normalizzato del campo `comment`. [Un esempio di parser è qui.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

L'esempio di parser può anche essere adattato per funzionare con HTML e trasformare i tag HTML negli elementi attesi da renderizzare per la tua piattaforma. 

### Tagging

Quando gli utenti vengono taggati in un commento, le informazioni vengono memorizzate in una lista chiamata `mentions`. Ogni oggetto in quella lista
ha la seguente struttura.

[inline-code-attrs-start title = 'Oggetto delle Menzioni del Commento'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id utente. Per gli utenti SSO, questo avrà prefissato il tuo id tenant. **/
    id: string
    /** Il testo finale del tag @mention, incluso il simbolo @. **/
    tag: string
    /** Il testo originale del tag @mention, incluso il simbolo @. **/
    rawTag: string
    /** Che tipo di utente è stato taggato. user = account FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se l'utente rinuncia alle notifiche, questo sarà comunque impostato su true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Quando gli hashtag vengono usati e analizzati correttamente, le informazioni vengono memorizzate in una lista chiamata `hashTags`. Ogni oggetto in quella lista
ha la seguente struttura. Gli hashtag possono anche essere aggiunti manualmente all'array `hashTags` del commento per le query, se `retain` è impostato.

[inline-code-attrs-start title = 'Oggetto HashTag del Commento'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** L'id dell'hashtag. **/
    id: string
    /** Il testo finale del tag #hashtag, incluso il simbolo #. **/
    tag: string
    /** Se l'hashtag è associato a un URL personalizzato, questo sarà definito. **/
    url?: string
    /** Se dobbiamo mantenere l'hashtag, anche se non esiste nel testo del commento, quando il commento viene aggiornato. Utile per taggare commenti senza cambiare il testo del commento. **/
    retain?: boolean
}
[inline-code-end]

---