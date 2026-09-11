The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment-objektets struktur

##### The "Create" Event Structure
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

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
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

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

#### HTTP-metoder

Du kan konfigurere HTTP-metoden for hver webhook-begivenhedstype i administrationspanelet:

- **Create-begivenhed**: POST eller PUT (standard: PUT)
- **Update-begivenhed**: POST eller PUT (standard: PUT)
- **Delete-begivenhed**: DELETE, POST eller PUT (standard: DELETE)

Da alle anmodninger indeholder et ID, er Create- og Update-operationer idempotente som standard (PUT). Gentagelse af den samme Create- eller Update-anmodning bør ikke oprette duplikerede objekter på din side.

#### Anmodnings‑headers

Hver webhook-anmodning inkluderer følgende headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Din API-hemmelighed |
| `X-FastComments-Timestamp` | Unix-tidsstempel (sekunder) da anmodningen blev signeret |
| `X-FastComments-Signature` | HMAC-SHA256-signatur (`sha256=<hex>`) |

Se [Sikkerhed & API‑tokens](/guide-webhooks.html#webhooks-api-tokens) for information om verifikation af HMAC-signaturen.