The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### La struttura dell'oggetto WebhookComment

##### Struttura dell'evento "Create"
The "create" event request body is a WebhookComment object.

##### Struttura dell'evento "Update"
The "update" event request body is a WebhookComment object.

##### Struttura dell'evento "Delete"
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Modifica a partire dal 14 novembre 2023. In precedenza il corpo della richiesta dell'evento "delete" conteneva solo l'ID del commento. Ora contiene il commento completo al momento dell'eliminazione.

Ogni chiave è sempre presente nel corpo. Quando il commento non ha valore per un campo, il corpo contiene `null` (o `false` per i booleani e `[]` per le liste), quindi la forma di una consegna non varia mai da un commento all'altro.

[inline-code-attrs-start title = 'L\'oggetto WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** L'ID del commento. **/
    id: string
    /** L'ID o l'URL che identifica il thread del commento. Normalizzato. **/
    urlId: string
    /** L'URL che punta a dove è stato lasciato il commento. **/
    url: string | null
    /** L'ID utente che ha lasciato il commento. Se SSO, prefisso con l'ID del tenant. **/
    userId: string | null
    /** L'email dell'utente che ha lasciato il commento. **/
    commenterEmail: string | null
    /** Il nome dell'utente mostrato nel widget del commento. Con SSO, può essere displayName. **/
    commenterName: string
    /** Testo grezzo del commento. **/
    comment: string
    /** Testo del commento dopo il parsing. **/
    commentHTML: string
    /** ID esterno del commento. **/
    externalId: string | null
    /** L'ID del commento genitore. **/
    parentId: string | null
    /** La data UTC in cui è stato lasciato il commento. **/
    date: UTC_ISO_DateString
    /** Karma combinato (up - down) dei voti. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Vero se l'utente era connesso quando ha commentato, o ha verificato il commento, o ha verificato la sua sessione quando il commento è stato lasciato. **/
    verified: boolean
    /** La data UTC in cui il commento è stato verificato. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Se un moderatore ha contrassegnato il commento come revisionato. **/
    reviewed: boolean
    /** La posizione, o codifica base64, dell'avatar. Sarà base64 solo se quel valore è stato passato con SSO. **/
    avatarSrc: string | null
    /** Il commento è stato contrassegnato manualmente o automaticamente come spam? **/
    isSpam: boolean
    /** Il commento è stato contrassegnato automaticamente come spam? **/
    aiDeterminedSpam: boolean
    /** Ci sono immagini nel commento? **/
    hasImages: boolean
    /** Il numero di pagina su cui si trova il commento per l'ordinamento "Most Relevant". **/
    pageNumber: number | null
    /** Il numero di pagina su cui si trova il commento per l'ordinamento "Oldest First". **/
    pageNumberOF: number | null
    /** Il numero di pagina su cui si trova il commento per l'ordinamento "Newest First". **/
    pageNumberNF: number | null
    /** Il commento è stato approvato automaticamente o manualmente? **/
    approved: boolean
    /** Il codice locale (formato: en_us) dell'utente quando il commento è stato scritto. **/
    locale: string | null
    /** Le @mentions scritte nel commento che sono state analizzate con successo. Vuoto quando non ce ne sono. **/
    mentions: CommentUserMention[]
    /** Il dominio da cui proviene il commento. **/
    domain: string | null
    /** Gli ID dei gruppi di moderazione associati a questo commento. Vuoto quando non ce ne sono. **/
    moderationGroupIds: string[]
    /** I badge mostrati accanto al nome del commentatore su questo commento, al momento della creazione dell'evento. Vuoto quando non ce ne sono. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'L\'oggetto Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** L'ID utente. Per gli utenti SSO, avrà il tuo ID tenant prefisso. **/
    id: string
    /** Il testo finale del tag @mention, incluso il simbolo @. **/
    tag: string
    /** Il testo originale del tag @mention, incluso il simbolo @. **/
    rawTag: string
    /** Il tipo di utente taggato. user = account FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Se l'utente rinuncia alle notifiche, questo sarà comunque impostato a true. **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

[inline-code-attrs-start title = 'L\'oggetto Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** L'ID del badge assegnato a questo utente. **/
    id: string
    /** Il tipo di badge. **/
    type: number
    /** La descrizione del badge, mostrata al passaggio del mouse sul badge. **/
    description: string
    /** Il testo mostrato sul badge. **/
    displayLabel?: string | null
    /** L'immagine mostrata per il badge, se ne usa una. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Una classe CSS extra applicata al badge. **/
    cssClass?: string | null
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

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Il tuo segreto API |
| `X-FastComments-Timestamp` | Timestamp Unix (secondi) quando la richiesta è stata firmata |
| `X-FastComments-Signature` | Firma HMAC-SHA256 (`sha256=<hex>`) |

Vedi [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) per informazioni sulla verifica della firma HMAC.