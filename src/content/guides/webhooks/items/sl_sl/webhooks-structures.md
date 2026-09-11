The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Struktura objekta WebhookComment

##### Struktura dogodka "Create"
The "create" event request body is a WebhookComment object.

##### Struktura dogodka "Update"
The "update" event request body is a WebhookComment object.

##### Struktura dogodka "Delete"
The "delete" event request body is a WebhookComment object.

    Sprememba od 14. novembra 2023
    Prej je telo zahteve za dogodek "delete" vsebovalo le ID komentarja. Zdaj vsebuje celoten komentar v času brisanja.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentarja. **/
    id: string
    /** ID ali URL, ki identificira nit komentarjev. Normalizirano. **/
    urlId: string
    /** URL, ki kaže na mesto, kjer je bil komentar objavljen. **/
    url: string | null
    /** ID uporabnika, ki je napisal komentar. Če je SSO, je predponjen z ID najemnika. **/
    userId: string | null
    /** E‑mail uporabnika, ki je napisal komentar. **/
    commenterEmail: string | null
    /** Ime uporabnika, ki se prikaže v pripomočku za komentarje. Pri SSO je lahko displayName. **/
    commenterName: string
    /** Surovo besedilo komentarja. **/
    comment: string
    /** Besedilo komentarja po razčlenitvi. **/
    commentHTML: string
    /** Zunanji ID komentarja. **/
    externalId: string | null
    /** ID nadrejenega komentarja. **/
    parentId: string | null
    /** Datum v UTC, ko je bil komentar objavljen. **/
    date: UTC_ISO_DateString
    /** Skupna karma (glasovi + - -). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Resnično, če je bil uporabnik prijavljen, ko je komentiral, ali je potrdil komentar, ali je potrdil sejo, ko je bil komentar objavljen. **/
    verified: boolean
    /** Datum v UTC, ko je bil komentar potrjen. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Če je moderator označil komentar kot pregledan. **/
    reviewed: boolean
    /** Lokacija ali base64 kodiranje avatarja. Base64 bo le, če je bila to vrednost posredovana z SSO. **/
    avatarSrc: string | null
    /** Ali je bil komentar ročno ali samodejno označen kot neželen? **/
    isSpam: boolean
    /** Ali je bil komentar samodejno označen kot neželen? **/
    aiDeterminedSpam: boolean
    /** Ali komentar vsebuje slike? **/
    hasImages: boolean
    /** Številka strani, na kateri je komentar pri razvrščanju po "Najbolj relevantnih". **/
    pageNumber: number | null
    /** Številka strani, na kateri je komentar pri razvrščanju po "Najstarejših najprej". **/
    pageNumberOF: number | null
    /** Številka strani, na kateri je komentar pri razvrščanju po "Najnovejših najprej". **/
    pageNumberNF: number | null
    /** Ali je bil komentar odobren samodejno ali ročno? **/
    approved: boolean
    /** Koda jezika (format: en_us) uporabnika, ko je bil komentar napisan. **/
    locale: string | null
    /** Oznake @, zapisane v komentarju, ki so bile uspešno razčlenjene. Prazno, ko jih ni. **/
    mentions: CommentUserMention[]
    /** Domena, iz katere je komentar. **/
    domain: string | null
    /** ID-ji moderacijskih skupin, povezani s tem komentarjem. Prazno, ko jih ni. **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Objekt Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID uporabnika. Za SSO uporabnike bo predponjen z ID najemnika. **/
    id: string
    /** Končni besedilni niz @omenjanja, vključno s simbolom @. **/
    tag: string
    /** Izvirni besedilni niz @omenjanja, vključno s simbolom @. **/
    rawTag: string
    /** Kakšna vrsta uporabnika je bila označena. user = račun FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Če se uporabnik odkloni od obvestil, bo to še vedno nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP metode

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST or PUT (default: PUT) -> **Create Event**: POST ali PUT (privzeto: PUT)
- **Update Event**: POST or PUT (default: PUT) -> **Update Event**: POST ali PUT (privzeto: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE) -> **Delete Event**: DELETE, POST ali PUT (privzeto: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### Glave zahteve

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaše API skrivnost |
| `X-FastComments-Timestamp` | Unix časovni žig (sekunde), ko je bila zahteva podpisana |
| `X-FastComments-Signature` | HMAC-SHA256 podpis (`sha256=<hex>`) |

See [Varnost in API žetoni](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.