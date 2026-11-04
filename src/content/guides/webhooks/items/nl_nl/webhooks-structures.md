De enige structuur die via webhooks wordt verstuurd is het WebhookComment object, hieronder uitgelijnd in TypeScript.

#### De WebhookComment Objectstructuur

##### De "Create" Eventstructuur
De request body van het "create" event is een WebhookComment object.

##### De "Update" Eventstructuur
De request body van het "update" event is een WebhookComment object.

##### De "Delete" Eventstructuur
De request body van het "delete" event is een WebhookComment object.

    Wijziging per 14 november 2023
    Voorheen bevatte de request body van het "delete" event alleen de comment id. Deze bevat nu de volledige comment op het moment van verwijdering.


Elke key is altijd aanwezig in de body. Wanneer de comment geen waarde heeft voor een veld bevat de body `null`
(of `false` voor booleans en `[]` voor lijsten), zodat de vorm van een levering nooit verschilt van de ene comment naar de andere.

[inline-code-attrs-start title = 'Het WebhookComment Object'; type = 'typescript'; inline-code-attrs-end]
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

Wanneer gebruikers getagd worden in een comment, wordt de informatie opgeslagen in een lijst genaamd `mentions`. Elk object in die lijst
heeft de volgende structuur.

[inline-code-attrs-start title = 'Het Webhook Mentions Object'; type = 'typescript'; inline-code-attrs-end]
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

Elke entry in de `badges` lijst heeft de volgende structuur. Zie de [Badges gids](/guide-badges.html) voor de badge types.

[inline-code-attrs-start title = 'Het Webhook Badge Object'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP Methoden

Je kunt de HTTP-methode voor elk webhook event type configureren in het admin paneel:

- **Create Event**: POST of PUT (standaard: PUT)
- **Update Event**: POST of PUT (standaard: PUT)
- **Delete Event**: DELETE, POST of PUT (standaard: DELETE)

Aangezien alle requests een ID bevatten, zijn Create- en Update-operaties standaard idempotent (PUT). Het herhalen van dezelfde Create- of Update-request zou aan jouw kant geen dubbele objecten mogen creëren.

#### Request Headers

Elke webhook request bevat de volgende headers:

| Header | Beschrijving |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Je API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconden) waarop de request is ondertekend |
| `X-FastComments-Signature` | HMAC-SHA256 handtekening (`sha256=<hex>`) |

Zie [Beveiliging & API Tokens](/guide-webhooks.html#webhooks-api-tokens) voor informatie over het verifiëren van de HMAC handtekening.