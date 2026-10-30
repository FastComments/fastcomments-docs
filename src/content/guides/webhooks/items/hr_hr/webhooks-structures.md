The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Struktura objekta WebhookComment

##### Struktura događaja "Create"
The "create" event request body is a WebhookComment object.

##### Struktura događaja "Update"
The "update" event request body is a WebhookComment object.

##### Struktura događaja "Delete"
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Promjena od 14. studenog 2023.
Prethodno je tijelo zahtjeva za događaj "delete" sadržavalo samo ID komentara. Sada sadrži cijeli komentar u trenutku brisanja.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

Svaki ključ je uvijek prisutan u tijelu. Kada komentar nema vrijednost za neko polje, tijelo nosi `null` (ili `false` za boolean vrijednosti i `[]` za liste), tako da oblik isporuke nikada ne varira od jednog komentara do drugog.

[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentara. **/
    id: string
    /** ID ili URL koji identificira nit komentara. Normalizirano. **/
    urlId: string
    /** URL koji pokazuje gdje je komentar ostavljen. **/
    url: string | null
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiksiran s ID-om najmoda. **/
    userId: string | null
    /** E‑mail korisnika koji je ostavio komentar. **/
    commenterEmail: string | null
    /** Ime korisnika koje se prikazuje u widgetu za komentar. S SSO-om, može biti displayName. **/
    commenterName: string
    /** Sirovi tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Vanjski ID komentara. **/
    externalId: string | null
    /** ID nadređenog komentara. **/
    parentId: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirani karma (up - down) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** Istina ako je korisnik bio prijavljen kada je komentirao, ili je verificirao komentar, ili je verificirao sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** UTC datum kada je komentar verificiran. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija ili base64 kodiranje avatara. Bit će base64 samo ako je to vrijednost proslijeđena s SSO. **/
    avatarSrc: string | null
    /** Je li je komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Je li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Postoje li slike u komentaru? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Most Relevant". **/
    pageNumber: number | null
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Oldest First". **/
    pageNumberOF: number | null
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Newest First". **/
    pageNumberNF: number | null
    /** Je li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod jezika (format: en_us) korisnika kada je komentar napisan. **/
    locale: string | null
    /** Spominjanja @ napisane u komentaru koja su uspješno parsirana. Prazno kada ih nema. **/
    mentions: CommentUserMention[]
    /** Domena iz koje je komentar. **/
    domain: string | null
    /** ID‑ovi moderacijskih grupa povezani s ovim komentarom. Prazno kada ih nema. **/
    moderationGroupIds: string[]
    /** Značke prikazane uz ime komentatora na ovom komentaru, u trenutku kada je događaj kreiran. Prazno kada ih nema. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

Kada su korisnici označeni u komentaru, informacije se pohranjuju u popisu pod nazivom `mentions`. Svaki objekt u tom popisu ima sljedeću strukturu.

[inline-code-attrs-start title = 'Objekt spominjanja Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, bit će prefiksiran s ID-om vašeg najmoda. **/
    id: string
    /** Završni tekst @mention oznake, uključujući @ simbol. **/
    tag: string
    /** Originalni tekst @mention oznake, uključujući @ simbol. **/
    rawTag: string
    /** Koja je vrsta korisnika označena. user = FastComments.com račun. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako korisnik odustane od obavijesti, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

Svaki unos u popisu `badges` ima sljedeću strukturu. Pogledajte [vodič za značke](/guide-badges.html) za vrste znački.

[inline-code-attrs-start title = 'Objekt značke Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** ID značke dodijeljene ovom korisniku. **/
    id: string
    /** Vrsta značke. **/
    type: number
    /** Opis značke, prikazan pri prelasku mišem preko značke. **/
    description: string
    /** Tekst prikazan na znački. **/
    displayLabel?: string | null
    /** Slika prikazana za značku, ako je koristi. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Dodatna CSS klasa primijenjena na značku. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP metode

You can configure the HTTP method for each webhook event type in the admin panel:

Možete konfigurirati HTTP metodu za svaki tip webhook događaja u administratorskom panelu:

- **Create Event**: POST ili PUT (zadano: PUT)
- **Update Event**: POST ili PUT (zadano: PUT)
- **Delete Event**: DELETE, POST ili PUT (zadano: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

Budući da svi zahtjevi sadrže ID, operacije Create i Update su po defaultu idempotentne (PUT). Ponovljeni isti Create ili Update zahtjev ne bi trebao stvoriti duple objekte na vašoj strani.

#### Zaglavlja zahtjeva

Each webhook request includes the following headers:

Svaki webhook zahtjev uključuje sljedeća zaglavlja:

| Zaglavlje | Opis |
|-----------|------|
| `Content-Type` | `application/json` |
| `token` | Vaša API tajna |
| `X-FastComments-Timestamp` | Unix vremenski žig (sekunde) kada je zahtjev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.

Pogledajte [Sigurnost i API tokeni](/guide-webhooks.html#webhooks-api-tokens) za informacije o provjeri HMAC potpisa.

---