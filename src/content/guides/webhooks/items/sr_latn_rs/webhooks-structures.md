Jedina struktura koja se šalje putem webhook-ova je WebhookComment objekat, prikazan u TypeScript-u ispod.

#### Struktura objekta WebhookComment

##### Struktura događaja "create"
Telo zahteva za događaj "create" je WebhookComment objekat.

##### Struktura događaja "update"
Telo zahteva za događaj "update" je WebhookComment objekat.

##### Struktura događaja "delete"
Telo zahteva za događaj "delete" je WebhookComment objekat.

    Promena od 14. novembra 2023.
    Prethodno je telo zahteva za događaj "delete" sadržavalo samo id komentara. Sada sadrži ceo komentar u trenutku brisanja.


[inline-code-attrs-start title = 'Objekat WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentara. **/
    id: string
    /** Id ili URL koji identifikuje nit komentara. Normalizovano. **/
    urlId: string
    /** URL koji pokazuje gde je komentar ostavljen. **/
    url?: string
    /** Id korisnika koji je ostavio komentar. Ako je SSO, prefiksovan tenant id-jem. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Kod SSO, može biti displayName. **/
    commenterName: string
    /** Sirov tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Eksterni id komentara. **/
    externalId?: string
    /** Id roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinovani karma (up - down) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio prijavljen kada je komentarisao, ili je verifikovao komentar, ili je verifikovao svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verifikovan. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija, ili base64 enkodiranje avatara. Biće base64 samo ako je ta vrednost prosleđena sa SSO. **/
    avatarSrc?: string
    /** Da li je komentar označen kao spam ručno ili automatski? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li u komentaru ima slika? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za smer sortiranja "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za smer sortiranja "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za smer sortiranja "Newest First". **/
    pageNumberNF: number
    /** Da li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod lokalizacije (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** @mentions napisani u komentaru koji su uspešno parsirani. **/
    mentions?: CommentUserMention[]
    /** Domen odakle je komentar. **/
    domain?: string
    /** Opcionalna lista id-jeva moderacijskih grupa povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi nazvanoj `mentions`. Svaki objekat u toj listi
ima sledeću strukturu.

[inline-code-attrs-start title = 'Objekat Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id korisnika. Za SSO korisnike, biće prefiksovan vašim tenant id-jem. **/
    id: string
    /** Finalni tekst @mention taga, uključujući simbol @. **/
    tag: string
    /** Originalni tekst @mention taga, uključujući simbol @. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP metode

Možete konfigurisati HTTP metodu za svaki tip webhook događaja u administratorskom panelu:

- **Create Event**: POST ili PUT (podrazumevano: PUT)
- **Update Event**: POST ili PUT (podrazumevano: PUT)
- **Delete Event**: DELETE, POST, ili PUT (podrazumevano: DELETE)

Pošto svi zahtevi sadrže ID, Create i Update operacije su idempotentne po podrazumevanju (PUT). Ponavljanje istog Create ili Update zahteva ne bi trebalo da stvori duplikate objekata na vašoj strani.

#### Zaglavlja zahteva

Svaki webhook zahtev sadrži sledeća zaglavlja:

| Zaglavlje | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (sekunde) kada je zahtev bio potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) za informacije o verifikaciji HMAC potpisa.

---