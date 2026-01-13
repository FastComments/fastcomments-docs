L'unica struttura inviata tramite webhook è l'oggetto WebhookComment, descritto in TypeScript qui sotto.

#### La struttura dell'oggetto WebhookComment

##### La "Create" Event Structure
Il body della richiesta dell'evento "create" è un oggetto WebhookComment.

##### La "Update" Event Structure
Il body della richiesta dell'evento "update" è un oggetto WebhookComment.

##### La "Delete" Event Structure
Il body della richiesta dell'evento "delete" è un oggetto WebhookComment.

    Modifica a partire dal 14 Nov 2023
    In precedenza il body della richiesta dell'evento "delete" conteneva solo l'id del commento. Ora contiene il commento completo al momento dell'eliminazione.


[inline-code-attrs-start title = 'Oggetto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'id del commento. **/
    id: string
    /** L'id o l'URL che identifica il thread dei commenti. Normalizzato. **/
    urlId: string
    /** L'URL che punta al luogo dove è stato lasciato il commento. **/
    url?: string
    /** L'id utente che ha lasciato il commento. Se SSO, prefissato con l'id del tenant. **/
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
    /** True se l'utente era autenticato quando ha commentato, o se ha verificato il commento, o se ha verificato la sessione quando il commento è stato lasciato. **/
    verified: boolean
    /** Data in cui il commento è stato verificato. **/
    verifiedDate?: number
    /** Se un moderatore ha contrassegnato il commento come revisionato. **/
    reviewed: boolean
    /** La posizione, o codifica base64, dell'avatar. Sarà in base64 solo se questo è stato il valore passato con SSO. **/
    avatarSrc?: string
    /** Il commento è stato contrassegnato manualmente o automaticamente come spam? **/
    isSpam: boolean
    /** Il commento è stato automaticamente contrassegnato come spam? **/
    aiDeterminedSpam: boolean
    /** Ci sono immagini nel commento? **/
    hasImages: boolean
    /** Il numero di pagina su cui si trova il commento per l'ordinamento "Most Relevant". **/
    pageNumber: number
    /** Il numero di pagina su cui si trova il commento per l'ordinamento "Oldest First". **/
    pageNumberOF: number
    /** Il numero di pagina su cui si trova il commento per l'ordinamento "Newest First". **/
    pageNumberNF: number
    /** Il commento è stato approvato automaticamente o manualmente? **/
    approved: boolean
    /** Il codice locale (formato: en_us) dell'utente quando il commento è stato scritto. **/
    locale: string
    /** Le @mention scritte nel commento che sono state analizzate con successo. **/
    mentions?: CommentUserMention[]
    /** Il dominio da cui proviene il commento. **/
    domain?: string
    /** L'elenco opzionale di id dei gruppi di moderazione associati a questo commento. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Oggetto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id utente. Per gli utenti SSO, avrà prefissato l'id del tenant. **/
    id: string
    /** Il testo finale del tag @mention, inclusivo del simbolo @. **/
    tag: string
    /** Il testo originale del tag @mention, inclusivo del simbolo @. **/
    rawTag: string
    /** Che tipo di utente è stato taggato. user = account FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se l'utente rinuncia alle notifiche, questo sarà comunque impostato su true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

Puoi configurare il metodo HTTP per ciascun tipo di evento webhook nel pannello di amministrazione:

- **Evento "Create"**: POST o PUT (predefinito: PUT)
- **Evento "Update"**: POST o PUT (predefinito: PUT)
- **Evento "Delete"**: DELETE, POST o PUT (predefinito: DELETE)

Poiché tutte le richieste contengono un ID, le operazioni Create e Update sono idempotenti per impostazione predefinita (PUT). Ripetere la stessa richiesta di Create o Update non dovrebbe creare oggetti duplicati dalla tua parte.

#### Request Headers

Ogni richiesta webhook include le seguenti intestazioni:

| Header | Descrizione |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Il tuo API Secret |
| `X-FastComments-Timestamp` | Timestamp Unix (secondi) quando la richiesta è stata firmata |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Vedi [Sicurezza e token API](/guides/webhooks/webhooks-api-tokens) per informazioni sulla verifica della firma HMAC.