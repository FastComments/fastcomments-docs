Jedina struktura koja se šalje putem webhook-ova je objekat WebhookComment, prikazan u TypeScript-u ispod.

#### Struktura objekta WebhookComment

##### Struktura događaja "Create"
Telo zahteva za događaj "create" je objekat WebhookComment.

##### Struktura događaja "Update"
Telo zahteva za događaj "update" je objekat WebhookComment.

##### Struktura događaja "Delete"
Telo zahteva za događaj "delete" je objekat WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Objekat WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentara. **/
    id: string
    /** ID ili URL koji identifikuje nit komentara. Normalizovano. **/
    urlId: string
    /** URL koji pokazuje gdje je komentar ostavljen. **/
    url?: string
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiksovano tenant id-jem. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Kod SSO, može biti displayName. **/
    commenterName: string
    /** Originalan tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Eksterni ID komentara. **/
    externalId?: string
    /** ID roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinovana karma (za - protiv) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Tačno ako je korisnik bio prijavljen kada je komentarisao, ili je verifikovao komentar, ili je verifikovao svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verifikovan. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija, ili base64 enkodiranje avatara. Biće base64 samo ako je ta vrijednost poslana sa SSO. **/
    avatarSrc?: string
    /** Da li je komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li postoje slike u komentaru? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za smer sortiranja "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za smer sortiranja "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za smer sortiranja "Newest First". **/
    pageNumberNF: number
    /** Da li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod lokala (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** @pominjanja napisana u komentaru koja su uspješno parsirana. **/
    mentions?: CommentUserMention[]
    /** Domen odakle je komentar. **/
    domain?: string
    /** Opcionalna lista ID-jeva grupa za moderaciju povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Objekat pomena u webhook-u'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, biće prefiksovano vašim tenant id-jem. **/
    id: string
    /** Krajnji tekst @mention taga, uključujući simbol @. **/
    tag: string
    /** Originalni tekst @mention taga, uključujući simbol @. **/
    rawTag: string
    /** Koji tip korisnika je tagovan. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od obavještenja, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### Request Headers

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API tajni ključ |
| `X-FastComments-Timestamp` | Unix timestamp (sekunde) kada je zahtev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

See [Sigurnost i API tokeni](/guides/webhooks/webhooks-api-tokens) for information on verifying the HMAC signature.

---