Jedina struktura koja se šalje putem webhooks je objekat WebhookComment, prikazan u TypeScript-u ispod.

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
    /** URL koji pokazuje mesto gde je komentar ostavljen. **/
    url?: string
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiksovan tenant id-jem. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu za komentare. Sa SSO, može biti displayName. **/
    commenterName: string
    /** Neobrađeni tekst komentara. **/
    comment: string
    /** Tekst komentara posle parsiranja. **/
    commentHTML: string
    /** Eksterni ID komentara. **/
    externalId?: string
    /** ID roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinovana vrednost glasova (za - protiv). **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio ulogovan kada je komentarisao, ili ako je verifikovao komentar, ili ako je verifikovao sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verifikovan. **/
    verifiedDate?: number
    /** Da li je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija ili base64 kodiranje avatara. Biće base64 samo ako je ta vrednost poslata sa SSO. **/
    avatarSrc?: string
    /** Da li je komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li u komentaru ima slika? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Newest First". **/
    pageNumberNF: number
    /** Da li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod lokalizacije (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** @pominjanja napisani u komentaru koja su uspešno parsirana. **/
    mentions?: CommentUserMention[]
    /** Domen odakle je komentar. **/
    domain?: string
    /** Opcionalna lista ID-jeva moderacijskih grupa povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi koja se zove `mentions`. Svaki objekat u toj listi
ima sledeću strukturu.

[inline-code-attrs-start title = 'Objekat Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike biće prefiksovan vašim tenant id-jem. **/
    id: string
    /** Konačan tekst @mention taga, uključujući simbol @. **/
    tag: string
    /** Originalni tekst @mention taga, uključujući simbol @. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik isključi iz notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP metode

Možete konfigurisati HTTP metodu za svaki tip webhook događaja u administratorskom panelu:

- **Create Event**: POST ili PUT (podrazumevano: PUT)
- **Update Event**: POST ili PUT (podrazumevano: PUT)
- **Delete Event**: DELETE, POST, ili PUT (podrazumevano: DELETE)

Pošto svi zahtevi sadrže ID, Create i Update operacije su idempotentne po defaultu (PUT). Ponavljanje istog Create ili Update zahteva ne bi trebalo da kreira duplikate objekata kod vas.

#### Zaglavlja zahteva

Svaki webhook zahtev uključuje sledeća zaglavlja:

| Zaglavlje | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API Secret |
| `X-FastComments-Timestamp` | Unix vremenski pečat (sekunde) kada je zahtev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Bezbednost i API tokeni](/guides/webhooks/webhooks-api-tokens) za informacije o verifikaciji HMAC potpisa.