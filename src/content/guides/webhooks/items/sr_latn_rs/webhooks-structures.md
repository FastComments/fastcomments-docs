Jedina struktura koja se šalje putem webhukova je objekat WebhookComment, prikazan u TypeScript-u ispod.

#### Struktura objekta WebhookComment

##### Struktura "create" događaja
Telo zahteva "create" događaja je objekat WebhookComment.

##### Struktura "update" događaja
Telo zahteva "update" događaja je objekat WebhookComment.

##### Struktura "delete" događaja
Telo zahteva "delete" događaja je objekat WebhookComment.

    Promena od 14. novembra 2023.
    Ranije je telo zahteva "delete" događaja sadržalo samo id komentara. Sada sadrži ceo komentar u trenutku brisanja.


Svaki ključ je uvek prisutan u telu. Kada komentar nema vrednost za neko polje, telo nosi `null`
(ili `false` za boolean vrednosti i `[]` za liste), tako da oblik isporuke nikada ne varira od komentara do komentara.

[inline-code-attrs-start title = 'Objekat WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

Kada su korisnici označeni u komentaru, informacija se čuva u listi pod nazivom `mentions`. Svaki objekat u toj listi
ima sledeću strukturu.

[inline-code-attrs-start title = 'Objekat Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
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

Svaki unos u listi `badges` ima sledeću strukturu. Pogledajte [vodič za Badges](/guide-badges.html) za tipove bedževa.

[inline-code-attrs-start title = 'Objekat Webhook Badge'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP metode

Možete konfigurisati HTTP metodu za svaki tip webhook događaja u admin panelu:

- **Create Event**: POST ili PUT (podrazumevano: PUT)
- **Update Event**: POST ili PUT (podrazumevano: PUT)
- **Delete Event**: DELETE, POST ili PUT (podrazumevano: DELETE)

Pošto svi zahtevi sadrže ID, Create i Update operacije su idempotentne po podrazumevanoj postavci (PUT). Ponavljanje istog Create ili Update zahteva ne bi trebalo da kreira duplirane objekte na vašoj strani.

#### Zaglavlja zahteva

Svaki webhook zahtev uključuje sledeća zaglavlja:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

Pogledajte [Bezbednost i API tokeni](/guide-webhooks.html#webhooks-api-tokens) za informacije o verifikaciji HMAC potpisa.