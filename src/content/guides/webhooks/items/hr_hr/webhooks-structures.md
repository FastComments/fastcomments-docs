Jedina struktura poslana putem webhookova je objekt WebhookComment, prikazan u TypeScriptu u nastavku.

#### Struktura objekta WebhookComment

##### Struktura događaja "Create"
Tijelo zahtjeva za događaj "create" je objekt WebhookComment.

##### Struktura događaja "Update"
Tijelo zahtjeva za događaj "update" je objekt WebhookComment.

##### Struktura događaja "Delete"
Tijelo zahtjeva za događaj "delete" je objekt WebhookComment.

    Promjena od 14. studenog 2023.
    Prethodno je tijelo zahtjeva za događaj "delete" sadržavalo samo id komentara. Sada sadrži cijeli komentar u trenutku brisanja.


[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentara. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL koji pokazuje gdje je komentar ostavljen. **/
    url?: string
    /** Id korisnika koji je ostavio komentar. Ako je SSO, prefiksan s tenant id. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Kod SSO može biti displayName. **/
    commenterName: string
    /** Neobrađeni tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Vanjski id komentara. **/
    externalId?: string
    /** Id roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirani karma (up - down) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio prijavljen kad je komentirao, ili je verificirao komentar, ili je verificirao svoju sesiju kad je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verificiran. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija ili base64 kodiranje avatara. Bit će base64 samo ako je ta vrijednost poslana s SSO. **/
    avatarSrc?: string
    /** Je li komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Je li komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Ima li u komentaru slika? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Most Relevant". **/
    pageNumber: number
    /** Broj stranice za sortiranje "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice za sortiranje "Newest First". **/
    pageNumberNF: number
    /** Je li komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod lokalizacije (format: en_us) korisnika kad je komentar pisan. **/
    locale: string
    /** @mentions napisani u komentaru koji su uspješno parsirani. **/
    mentions?: CommentUserMention[]
    /** Domen od kojeg komentar potječe. **/
    domain?: string
    /** Opcionalna lista id-eva moderacijskih grupa povezanih s ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Objekt spominjanja webhooka'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id korisnika. Za SSO korisnike, bit će prefiksan vašim tenant id. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** Kakav tip korisnika je bio označen. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Metode

Možete konfigurirati HTTP metodu za svaku vrstu webhook događaja u administratorskom sučelju:

- **Create Event**: POST ili PUT (zadano: PUT)
- **Update Event**: POST ili PUT (zadano: PUT)
- **Delete Event**: DELETE, POST ili PUT (zadano: DELETE)

Budući da svi zahtjevi sadrže ID, operacije Create i Update su po defaultu idempotentne (PUT). Ponavljanje istog Create ili Update zahtjeva ne bi trebalo stvoriti duplicirane objekte na vašoj strani.

#### Zaglavlja zahtjeva

Svaki webhook zahtjev uključuje sljedeća zaglavlja:

| Zaglavlje | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API tajni ključ |
| `X-FastComments-Timestamp` | Unix timestamp (sekunde) kada je zahtjev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Sigurnost i API tokeni](/guide-webhooks.html#webhooks-api-tokens) za informacije o provjeri HMAC potpisa.

---