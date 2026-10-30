Den eneste struktur, der sendes via webhooks, er WebhookComment‑objektet, beskrevet i TypeScript nedenfor.

#### WebhookComment‑objektets struktur

##### "Create"-begivenhedens struktur  
The "create" event request body is a WebhookComment object.

##### "Update"-begivenhedens struktur  
The "update" event request body is a WebhookComment object.

##### "Delete"-begivenhedens struktur  
The "delete" event request body is a WebhookComment object.

Ændring pr. 14. november 2023  
Tidligere indeholdt anmodningskroppen for "delete"-begivenheden kun kommentar‑id'et. Den indeholder nu den fulde kommentar på tidspunktet for sletning.

Alle nøgler er altid til stede i kroppen. Når kommentaren ikke har en værdi for et felt, indeholder kroppen `null` (eller `false` for boolske værdier og `[]` for lister), så strukturen af en levering aldrig varierer fra en kommentar til den næste.

[inline-code-attrs-start title = 'WebhookComment-objektet'; type = 'typescript'; inline-code-attrs-end]
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

Når brugere er tagget i en kommentar, gemmes informationen i en liste kaldet `mentions`. Hvert objekt i den liste har følgende struktur.

[inline-code-attrs-start title = 'Webhook-mentions-objektet'; type = 'typescript'; inline-code-attrs-end]
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

Hvert element i `badges`‑listen har følgende struktur. Se [Badges guide](/guide-badges.html) for badge‑typerne.

[inline-code-attrs-start title = 'Webhook-badge-objektet'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP-metoder

Du kan konfigurere HTTP‑metoden for hver webhook‑begivenhedstype i adminpanelet:

- **Create‑begivenhed**: POST eller PUT (standard: PUT)
- **Update‑begivenhed**: POST eller PUT (standard: PUT)
- **Delete‑begivenhed**: DELETE, POST eller PUT (standard: DELETE)

Da alle anmodninger indeholder en ID, er Create‑ og Update‑operationer idempotente som standard (PUT). Gentagelse af den samme Create‑ eller Update‑anmodning bør ikke oprette dublerede objekter på din side.

#### Anmodnings‑headers

Hver webhook‑anmodning inkluderer følgende headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Din API‑hemmelighed |
| `X-FastComments-Timestamp` | Unix‑tidsstempel (sekunder) da anmodningen blev signeret |
| `X-FastComments-Signature` | HMAC‑SHA256‑signatur (`sha256=<hex>`) |

Se [Sikkerhed & API‑tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.