Edina struktura, poslana prek webhookov, je objekt WebhookComment, predstavljen v TypeScriptu spodaj.

#### Struktura objekta WebhookComment

##### Struktura dogodka "Create"
Request body dogodka "create" je objekt WebhookComment.

##### Struktura dogodka "Update"
Request body dogodka "update" je objekt WebhookComment.

##### Struktura dogodka "Delete"
Request body dogodka "delete" je objekt WebhookComment.

    Sprememba z dne 14. novembra 2023
    Pred tem je request body dogodka "delete" vseboval samo id komentarja. Zdaj vsebuje celoten komentar v času brisanja.


[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentarja. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL, ki kaže, kjer je bil komentar objavljen. **/
    url?: string
    /** ID uporabnika, ki je pustil komentar. Če je SSO, je predponan z tenant id. **/
    userId?: string
    /** E-pošta uporabnika, ki je pustil komentar. **/
    commenterEmail?: string
    /** Ime uporabnika, ki se prikaže v komentar-vidžetu. Pri SSO je lahko displayName. **/
    commenterName: string
    /** Surovo besedilo komentarja. **/
    comment: string
    /** Besedilo komentarja po parsiranju. **/
    commentHTML: string
    /** Zunanji ID komentarja. **/
    externalId?: string
    /** ID nadrejenega komentarja. **/
    parentId?: string | null
    /** UTC datum, ko je bil komentar oddan. **/
    date: UTC_ISO_DateString
    /** Kombinirana karma glasov (up - down). **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, če je bil uporabnik prijavljen, ko je komentiral, ali če je bil komentar overjen, ali če je ob oddaji komentarja preveril svojo sejo. **/
    verified: boolean
    /** Datum, ko je bil komentar overjen. **/
    verifiedDate?: number
    /** Če je moderator označil komentar kot pregledan. **/
    reviewed: boolean
    /** Lokacija ali base64 kodiranje avatarja. Bo base64 le, če je bila ta vrednost poslana z SSO. **/
    avatarSrc?: string
    /** Ali je bil komentar ročno ali samodejno označen kot spam? **/
    isSpam: boolean
    /** Ali je bil komentar samodejno označen kot spam? **/
    aiDeterminedSpam: boolean
    /** Ali so v komentarju slike? **/
    hasImages: boolean
    /** Številka strani, na kateri je komentar za vrstni red "Most Relevant". **/
    pageNumber: number
    /** Številka strani, na kateri je komentar za vrstni red "Oldest First". **/
    pageNumberOF: number
    /** Številka strani, na kateri je komentar za vrstni red "Newest First". **/
    pageNumberNF: number
    /** Ali je bil komentar odobren samodejno ali ročno? **/
    approved: boolean
    /** Koda lokalizacije (format: en_us) uporabnika, ko je bil komentar napisan. **/
    locale: string
    /** @omenitve, zapisane v komentarju, ki so bile uspešno parsirane. **/
    mentions?: CommentUserMention[]
    /** Domena, iz katere izvira komentar. **/
    domain?: string
    /** Neobvezen seznam ID-jev skupin moderacije, povezanih s tem komentarjem. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Ko so uporabniki omenjeni v komentarju, se informacije shranijo v seznamu z imenom `mentions`. Vsak objekt v tem seznamu ima naslednjo strukturo.

[inline-code-attrs-start title = 'Objekt Webhook omemb'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID uporabnika. Pri SSO uporabnikih bo predpona vaš tenant id. **/
    id: string
    /** Končno besedilo @mention oznake, vključno s simbolom @. **/
    tag: string
    /** Izvirno besedilo @mention oznake, vključno s simbolom @. **/
    rawTag: string
    /** Kakšne vrste uporabnik je bil omenjen. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Če se uporabnik odklopi od obvestil, bo to kljub temu nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods

Na administratorskem vmesniku lahko konfigurirate HTTP metodo za vsako vrsto webhook dogodka:

- **Create Event**: POST ali PUT (privzeto: PUT)
- **Update Event**: POST ali PUT (privzeto: PUT)
- **Delete Event**: DELETE, POST ali PUT (privzeto: DELETE)

Ker vse zahteve vsebujejo ID, sta operaciji Create in Update po privzetku idempotentni (PUT). Ponovitev iste zahteve Create ali Update ne bi smela ustvariti podvojenih objektov na vaši strani.

#### Request Headers

Vsaka webhook zahteva vključuje naslednje glave:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API skrivni ključ |
| `X-FastComments-Timestamp` | Unix časovni žig (sekunde), ko je bila zahteva podpisana |
| `X-FastComments-Signature` | HMAC-SHA256 podpis (`sha256=<hex>`) |

Za informacije o preverjanju HMAC podpisa glejte [Varnost in API žetoni](/guide-webhooks.html#webhooks-api-tokens).

---