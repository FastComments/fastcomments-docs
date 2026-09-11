Die einzige Struktur, die über Webhooks gesendet wird, ist das WebhookComment-Objekt, das unten in TypeScript dargestellt ist.

#### Die Struktur des WebhookComment-Objekts

##### Die Struktur des "Create"-Ereignisses
Der Anforderungstext des "create"-Ereignisses ist ein WebhookComment-Objekt.

##### Die Struktur des "Update"-Ereignisses
Der Anforderungstext des "update"-Ereignisses ist ein WebhookComment-Objekt.

##### Die Struktur des "Delete"-Ereignisses
Der Anforderungstext des "delete"-Ereignisses ist ein WebhookComment-Objekt.

    Änderung ab 14. November 2023
    Früher enthielt der Anforderungstext des "delete"-Ereignisses nur die Kommentar-ID. Jetzt enthält er den vollständigen Kommentar zum Zeitpunkt der Löschung.

Jeder Schlüssel ist immer im Body vorhanden. Wenn der Kommentar keinen Wert für ein Feld hat, enthält der Body `null` (oder `false` für Booleans und `[]` für Listen), sodass die Struktur einer Lieferung nie von einem Kommentar zum anderen variiert.

[inline-code-attrs-start title = 'Das WebhookComment-Objekt'; type = 'typescript'; inline-code-attrs-end]
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

Wenn Benutzer in einem Kommentar markiert werden, werden die Informationen in einer Liste namens `mentions` gespeichert. Jedes Objekt in dieser Liste hat die folgende Struktur.

[inline-code-attrs-start title = 'Das Webhook-Mentions-Objekt'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP-Methoden

Sie können die HTTP-Methode für jeden Webhook-Ereignistyp im Admin-Panel konfigurieren:

- **Create Event**: POST oder PUT (Standard: PUT)
- **Update Event**: POST oder PUT (Standard: PUT)
- **Delete Event**: DELETE, POST oder PUT (Standard: DELETE)

Da alle Anfragen eine ID enthalten, sind Create- und Update-Operationen standardmäßig (PUT) idempotent. Das Wiederholen derselben Create- oder Update-Anfrage sollte auf Ihrer Seite keine doppelten Objekte erzeugen.

#### Anforderungs-Header

Jede Webhook-Anfrage enthält die folgenden Header:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Ihr API-Geheimnis |
| `X-FastComments-Timestamp` | Unix-Zeitstempel (Sekunden), wenn die Anfrage signiert wurde |
| `X-FastComments-Signature` | HMAC-SHA256-Signatur (`sha256=<hex>`) |

Siehe [Sicherheit & API-Token](/guide-webhooks.html#webhooks-api-tokens) für Informationen zur Überprüfung der HMAC-Signatur.

---