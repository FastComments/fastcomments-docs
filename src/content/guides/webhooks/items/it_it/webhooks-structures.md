The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### La struttura dell'oggetto WebhookComment

##### Struttura dell'evento "Create"
Il corpo della richiesta dell'evento "create" è un oggetto WebhookComment.

##### Struttura dell'evento "Update"
Il corpo della richiesta dell'evento "update" è un oggetto WebhookComment.

##### Struttura dell'evento "Delete"
Il corpo della richiesta dell'evento "delete" è un oggetto WebhookComment.

    Modifica a partire dal 14 novembre 2023
    In precedenza il corpo della richiesta dell'evento "delete" conteneva solo l'ID del commento. Ora contiene il commento completo al momento dell'eliminazione.

Ogni chiave è sempre presente nel corpo. Quando il commento non ha valore per un campo, il corpo contiene `null` (o `false` per i booleani e `[]` per le liste), quindi la forma di una consegna non varia mai da un commento all'altro.

[inline-code-attrs-start title = 'L\'oggetto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
}
[inline-code-end]

Quando gli utenti sono taggati in un commento, le informazioni sono memorizzate in un elenco chiamato `mentions`. Ogni oggetto in quell'elenco ha la seguente struttura.

[inline-code-attrs-start title = 'L\'oggetto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

#### Metodi HTTP

Puoi configurare il metodo HTTP per ogni tipo di evento webhook nel pannello di amministrazione:

- **Evento Create**: POST o PUT (predefinito: PUT)
- **Evento Update**: POST o PUT (predefinito: PUT)
- **Evento Delete**: DELETE, POST o PUT (predefinito: DELETE)

Poiché tutte le richieste contengono un ID, le operazioni Create e Update sono idempotenti per impostazione predefinita (PUT). Ripetere la stessa richiesta Create o Update non dovrebbe creare oggetti duplicati sul tuo lato.

#### Intestazioni della richiesta

Ogni richiesta webhook include le seguenti intestazioni:

| Intestazione | Descrizione |
|--------------|-------------|
| `Content-Type` | `application/json` |
| `token` | Il tuo segreto API |
| `X-FastComments-Timestamp` | Timestamp Unix (secondi) quando la richiesta è stata firmata |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Vedi [Sicurezza e token API](/guide-webhooks.html#webhooks-api-tokens) per informazioni su come verificare la firma HMAC.