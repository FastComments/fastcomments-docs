Die einzige Struktur, die über Webhooks gesendet wird, ist das WebhookComment‑Objekt, das unten in TypeScript dargestellt ist.

#### Die Struktur des WebhookComment‑Objekts

##### Die „Create“-Ereignisstruktur
Der Anfragetext des „create“-Ereignisses ist ein WebhookComment‑Objekt.

##### Die „Update“-Ereignisstruktur
Der Anfragetext des „update“-Ereignisses ist ein WebhookComment‑Objekt.

##### Die „Delete“-Ereignisstruktur
Der Anfragetext des „delete“-Ereignisses ist ein WebhookComment‑Objekt.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Jeder Schlüssel ist immer im Body vorhanden. Wenn ein Kommentar keinen Wert für ein Feld hat, enthält der Body `null`
(oder `false` für Booleans und `[]` für Listen), sodass die Form einer Lieferung niemals von einem Kommentar zum nächsten variiert.

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
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
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

Jeder Eintrag in der `badges`‑Liste hat die folgende Struktur. Siehe den [Badges guide](/guide-badges.html) für die Badge‑Typen.

[inline-code-attrs-start title = 'Das Webhook-Badge-Objekt'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP‑Methoden

Sie können die HTTP‑Methode für jeden Webhook‑Ereignistyp im Admin‑Panel konfigurieren:

- **Create‑Ereignis**: POST oder PUT (Standard: PUT)
- **Update‑Ereignis**: POST oder PUT (Standard: PUT)
- **Delete‑Ereignis**: DELETE, POST oder PUT (Standard: DELETE)

Da alle Anfragen eine ID enthalten, sind Create‑ und Update‑Operationen standardmäßig idempotent (PUT). Das Wiederholen derselben Create‑ oder Update‑Anfrage sollte auf Ihrer Seite keine doppelten Objekte erzeugen.

#### Anforderungs‑Header

Jede Webhook‑Anfrage enthält die folgenden Header:

| Header | Beschreibung |
|--------|---------------|
| `Content-Type` | `application/json` |
| `token` | Ihr API‑Geheimnis |
| `X-FastComments-Timestamp` | Unix‑Zeitstempel (Sekunden) zum Zeitpunkt der Signatur |
| `X-FastComments-Signature` | HMAC‑SHA256‑Signatur (`sha256=<hex>`) |

Siehe [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) für Informationen zur Überprüfung der HMAC‑Signatur.