L'unica struttura inviata tramite webhook è l'oggetto WebhookComment, illustrato in TypeScript qui sotto.

#### La struttura dell'oggetto WebhookComment

##### Struttura dell'evento "create"
Il corpo della richiesta per l'evento "create" è un oggetto WebhookComment.

##### Struttura dell'evento "update"
Il corpo della richiesta per l'evento "update" è un oggetto WebhookComment.

##### Struttura dell'evento "delete"
Il corpo della richiesta per l'evento "delete" è un oggetto WebhookComment.

    Modifica del 14 novembre 2023
    Precedentemente il corpo della richiesta dell'evento "delete" conteneva solo l'id del commento. Ora contiene il commento completo al momento della cancellazione.


[inline-code-attrs-start title = 'Oggetto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'id del commento. **/
    id: string
    /** L'id o l'URL che identifica il thread di commento. Normalizzato. **/
    urlId: string
    /** L'URL che punta al luogo in cui è stato lasciato il commento. **/
    url?: string
    /** L'id utente che ha lasciato il commento. Se SSO, viene prefissato con l'id del tenant. **/
    userId?: string
    /** L'email dell'utente che ha lasciato il commento. **/
    commenterEmail?: string
    /** Il nome dell'utente mostrato nel widget dei commenti. In caso di SSO, può essere displayName. **/
    commenterName: string
    /** Testo grezzo del commento. **/
    comment: string
    /** Testo del commento dopo il parsing. **/
    commentHTML: string
    /** Id esterno del commento. **/
    externalId?: string
    /** L'id del commento padre. **/
    parentId?: string | null
    /** La data UTC in cui è stato lasciato il commento. **/
    date: UTC_ISO_DateString
    /** Karma combinato (up - down) dei voti. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True se l'utente era autenticato quando ha commentato, o se ha verificato il commento, o se ha verificato la sessione quando è stato lasciato il commento. **/
    verified: boolean
    /** Data in cui il commento è stato verificato. **/
    verifiedDate?: number
    /** Se un moderatore ha contrassegnato il commento come revisionato. **/
    reviewed: boolean
    /** La posizione, o la codifica base64, dell'avatar. Sarà base64 solo se questo valore è stato passato con SSO. **/
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
    /** Il codice locale (formato: en_us) dell'utente quando il commento è stato scritto. **/
    locale: string
    /** Le @mentions presenti nel commento che sono state analizzate con successo. **/
    mentions?: CommentUserMention[]
    /** Il dominio da cui proviene il commento. **/
    domain?: string
    /** L'elenco opzionale di id dei gruppi di moderazione associati a questo commento. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Quando gli utenti vengono taggati in un commento, le informazioni sono memorizzate in una lista chiamata `mentions`. Ogni oggetto in quella lista ha la seguente struttura.

[inline-code-attrs-start title = 'Oggetto Mentions del Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'id utente. Per gli utenti SSO, avrà il prefisso dell'id del tenant. **/
    id: string
    /** Il testo finale del tag @mention, inclusivo del simbolo @. **/
    tag: string
    /** Il testo originale del tag @mention, inclusivo del simbolo @. **/
    rawTag: string
    /** Tipo di utente taggato. user = account FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se l'utente rinuncia alle notifiche, questo sarà comunque impostato su true. **/
    sent: boolean
}
[inline-code-end]

#### Metodi HTTP

Puoi configurare il metodo HTTP per ciascun tipo di evento webhook nel pannello di amministrazione:

- **Evento "create"**: POST o PUT (predefinito: PUT)
- **Evento "update"**: POST o PUT (predefinito: PUT)
- **Evento "delete"**: DELETE, POST o PUT (predefinito: DELETE)

Poiché tutte le richieste contengono un ID, le operazioni di Create e Update sono idempotenti per impostazione predefinita (PUT). Ripetere la stessa richiesta di Create o Update non dovrebbe creare oggetti duplicati sul vostro sistema.

#### Intestazioni della richiesta

Ogni richiesta webhook include le seguenti intestazioni:

| Header | Descrizione |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Il tuo API Secret |
| `X-FastComments-Timestamp` | Timestamp Unix (secondi) al momento in cui la richiesta è stata firmata |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Vedi [Sicurezza e Token API](/guide-webhooks.html#webhooks-api-tokens) per informazioni sulla verifica della firma HMAC.