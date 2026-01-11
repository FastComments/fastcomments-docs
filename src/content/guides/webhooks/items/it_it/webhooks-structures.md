L'unica struttura inviata tramite webhook è l'oggetto WebhookComment, descritto in TypeScript di seguito.

#### The WebhookComment Object Structure

##### The "Create" Event Structure
Il corpo della richiesta dell'evento "create" è un oggetto WebhookComment.

##### The "Update" Event Structure
Il corpo della richiesta dell'evento "update" è un oggetto WebhookComment.

##### The "Delete" Event Structure
Il corpo della richiesta dell'evento "delete" è un oggetto WebhookComment.

    Modifica del 14 novembre 2023
    In precedenza il corpo della richiesta dell'evento "delete" conteneva solo l'id del commento. Ora contiene il commento completo al momento dell'eliminazione.


[inline-code-attrs-start title = 'Oggetto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'id del commento. **/
    id: string
    /** L'id o l'URL che identifica il thread del commento. Normalizzato. **/
    urlId: string
    /** L'URL che indica dove è stato lasciato il commento. **/
    url?: string
    /** L'id utente che ha lasciato il commento. Se SSO, preceduto dall'id del tenant. **/
    userId?: string
    /** L'email dell'utente che ha lasciato il commento. **/
    commenterEmail?: string
    /** Il nome dell'utente mostrato nel widget dei commenti. Con SSO, può essere displayName. **/
    commenterName: string
    /** Testo grezzo del commento. **/
    comment: string
    /** Testo del commento dopo il parsing. **/
    commentHTML: string
    /** Id esterno del commento. **/
    externalId?: string
    /** L'id del commento genitore. **/
    parentId?: string | null
    /** La data UTC in cui il commento è stato lasciato. **/
    date: UTC_ISO_DateString
    /** Karma combinato (up - down) dei voti. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True se l'utente era loggato quando ha commentato, o ha verificato il commento, o se ha verificato la sua sessione quando il commento è stato lasciato. **/
    verified: boolean
    /** Data in cui il commento è stato verificato. **/
    verifiedDate?: number
    /** Se un moderatore ha contrassegnato il commento come revisionato. **/
    reviewed: boolean
    /** La posizione, o codifica base64, dell'avatar. Sarà base64 solo se quel valore è stato passato con SSO. **/
    avatarSrc?: string
    /** Il commento è stato contrassegnato come spam manualmente o automaticamente? **/
    isSpam: boolean
    /** Il commento è stato contrassegnato automaticamente come spam? **/
    aiDeterminedSpam: boolean
    /** Ci sono immagini nel commento? **/
    hasImages: boolean
    /** Il numero di pagina in cui si trova il commento per l'ordinamento "Most Relevant". **/
    pageNumber: number
    /** Il numero di pagina in cui si trova il commento per l'ordinamento "Oldest First". **/
    pageNumberOF: number
    /** Il numero di pagina in cui si trova il commento per l'ordinamento "Newest First". **/
    pageNumberNF: number
    /** Il commento è stato approvato automaticamente o manualmente? **/
    approved: boolean
    /** Il codice della localizzazione (formato: en_us) dell'utente quando il commento è stato scritto. **/
    locale: string
    /** Le @mention scritte nel commento che sono state analizzate correttamente. **/
    mentions?: CommentUserMention[]
    /** Il dominio da cui proviene il commento. **/
    domain?: string
    /** L'elenco opzionale degli id dei gruppi di moderazione associati a questo commento. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Oggetto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id dell'utente. Per gli utenti SSO, avrà il tenant id come prefisso. **/
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

#### HTTP Methods Used

**Create e Update usano entrambi HTTP PUT e non POST!**

Poiché tutte le nostre richieste contengono un ID, ripetere la stessa richiesta Create o Update non dovrebbe creare nuovi oggetti dalla vostra parte.

Ciò significa che queste chiamate sono idempotenti e dovrebbero essere eventi PUT secondo la specifica HTTP.

---