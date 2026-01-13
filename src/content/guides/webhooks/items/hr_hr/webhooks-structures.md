Jedina struktura koja se šalje putem webhookova je objekt WebhookComment, prikazan u TypeScriptu u nastavku.

#### Struktura objekta WebhookComment

##### Struktura događaja "create"
Tijelo zahtjeva za događaj "create" je objekt WebhookComment.

##### Struktura događaja "update"
Tijelo zahtjeva za događaj "update" je objekt WebhookComment.

##### Struktura događaja "delete"
Tijelo zahtjeva za događaj "delete" je objekt WebhookComment.

    Promjena od 14. studenog 2023.
    Ranije je tijelo zahtjeva za događaj "delete" sadržavalo samo id komentara. Sada sadrži puni komentar u trenutku brisanja.


[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentara. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** URL koji upućuje na mjesto gdje je komentar ostavljen. **/
    url?: string
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiksan s tenant id. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Kod SSO može biti displayName. **/
    commenterName: string
    /** Neobrađeni tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Vanjski ID komentara. **/
    externalId?: string
    /** ID roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirani karma (za - protiv) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio prijavljen kad je komentirao, ili su potvrdili komentar, ili su potvrdili svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar potvrđen. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija ili base64 enkodiranje avatara. Bit će base64 samo ako je ta vrijednost proslijeđena s SSO. **/
    avatarSrc?: string
    /** Je li komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Je li komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Ima li u komentaru slika? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Newest First". **/
    pageNumberNF: number
    /** Je li komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kôd lokalizacije (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** @mentioni navedeni u komentaru koji su uspješno parsirani. **/
    mentions?: CommentUserMention[]
    /** Domen odakle je komentar. **/
    domain?: string
    /** Opcionalna lista ID-eva grupa za moderaciju povezanih s ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Objekt spominjanja u webhooku'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike bit će prefiksan vašim tenant id. **/
    id: string
    /** Konačni tekst @mention taga, uključujući simbol @. **/
    tag: string
    /** Izvorni tekst @mention taga, uključujući simbol @. **/
    rawTag: string
    /** Koja vrsta korisnika je označena. user = FastComments.com račun. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od obavijesti, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP metode

Možete konfigurirati HTTP metodu za svaku vrstu webhook događaja u administratorskom sučelju:

- **Create Event**: POST ili PUT (zadano: PUT)
- **Update Event**: POST ili PUT (zadano: PUT)
- **Delete Event**: DELETE, POST, ili PUT (zadano: DELETE)

Budući da svi zahtjevi sadrže ID, operacije Create i Update su po defaultu idempotentne (PUT). Ponavljanje istog Create ili Update zahtjeva ne bi trebalo stvarati duplikate objekata na vašoj strani.

#### Zaglavlja zahtjeva

Svaki webhook zahtjev uključuje sljedeća zaglavlja:

| Zaglavlje | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API Secret |
| `X-FastComments-Timestamp` | Unix vremenska oznaka (sekunde) kada je zahtjev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Sigurnost i API tokeni](/guides/webhooks/webhooks-api-tokens) za informacije o provjeri HMAC potpisa.

---