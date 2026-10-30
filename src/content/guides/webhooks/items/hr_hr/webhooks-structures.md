The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### The WebhookComment Object Structure

##### The "Create" Event Structure
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Promjena od 14. studenog 2023.  
Prethodno je tijelo zahtjeva za događaj "delete" sadržavalo samo ID komentara. Sada sadrži cijeli komentar u trenutku brisanja.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

Svaki ključ je uvijek prisutan u tijelu. Kada komentar nema vrijednost za polje, tijelo nosi `null`  
(ili `false` za boolean vrijednosti i `[]` za liste), tako da oblik isporuke nikada ne varira od jednog komentara do drugog.

[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
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

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

Kada su korisnici označeni u komentaru, informacije se pohranjuju u popisu pod nazivom `mentions`. Svaki objekt u tom popisu
ima sljedeću strukturu.

[inline-code-attrs-start title = 'Objekt spominjanja Webhook'; type = 'typescript'; inline-code-attrs-end]
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

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

Svaki unos u popisu `badges` ima sljedeću strukturu. Pogledajte [vodič za značke](/guide-badges.html) za vrste znački.

[inline-code-attrs-start title = 'Objekt značke Webhook'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP Methods

Možete konfigurirati HTTP metodu za svaki tip webhook događaja u administratorskom panelu:

- **Create Event**: POST ili PUT (zadano: PUT)
- **Update Event**: POST ili PUT (zadano: PUT)
- **Delete Event**: DELETE, POST ili PUT (zadano: DELETE)

Budući da svi zahtjevi sadrže ID, operacije Create i Update su po zadanim postavkama idempotentne (PUT). Ponovno slanje istog Create ili Update zahtjeva ne bi trebalo stvoriti duple objekte na vašoj strani.

#### Request Headers

Svaki webhook zahtjev uključuje sljedeća zaglavlja:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaša API tajna |
| `X-FastComments-Timestamp` | Unix vremenski žig (sekunde) kada je zahtjev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Sigurnost i API tokeni](/guide-webhooks.html#webhooks-api-tokens) za informacije o provjeri HMAC potpisa.