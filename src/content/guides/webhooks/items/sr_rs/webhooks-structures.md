Jedina struktura koja se šalje putem webhook-ova je objekat WebhookComment, prikazan u TypeScript-u ispod.

#### Struktura objekta WebhookComment

##### Struktura događaja "create"
Telo zahteva za događaj "create" je objekat WebhookComment.

##### Struktura događaja "update"
Telo zahteva za događaj "update" je objekat WebhookComment.

##### Struktura događaja "delete"
Telo zahteva za događaj "delete" je objekat WebhookComment.

    Promena od 14. novembra 2023.
    Ranije je telo zahteva za događaj "delete" sadržavalo samo id komentara. Sada sadrži ceo komentar u trenutku brisanja.


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
    /** Ime korisnika koje se prikazuje u widgetu za komentare. Kod SSO, može biti displayName. **/
    commenterName: string
    /** Sirovi tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Eksterni id komentara. **/
    externalId?: string
    /** Id roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinovana karma (up - down) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio prijavljen kada je komentarisao, ili je verifikovao komentar, ili je verifikovao svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verifikovan. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija ili base64 enkodovanje avatara. Biće base64 samo ako je ta vrednost prosleđena sa SSO. **/
    avatarSrc?: string
    /** Da li je komentar manuelno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li postoje slike u komentaru? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za sortiranje "Newest First". **/
    pageNumberNF: number
    /** Da li je komentar odobren automatski ili manuelno? **/
    approved: boolean
    /** Kod lokaliteta (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** Domen odakle je komentar. **/
    domain?: string
    /** Opcionalna lista id-jeva grupa za moderaciju povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi nazvanoj `mentions`. Svaki objekat u toj listi
ima sledeću strukturu.

[inline-code-attrs-start title = 'Objekat pomena Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id korisnika. Za SSO korisnike biće prefiksovan vašim tenant id-jem. **/
    id: string
    /** Konačan tekst @mention taga, uključujući simbol @. **/
    tag: string
    /** Originalni tekst @mention taga, uključujući simbol @. **/
    rawTag: string
    /** Ko je tip korisnika koji je označen. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP metode

Možete konfigurisati HTTP metodu za svaki tip webhook događaja u administratorskom panelu:

- **Create Event**: POST ili PUT (podrazumevano: PUT)
- **Update Event**: POST ili PUT (podrazumevano: PUT)
- **Delete Event**: DELETE, POST ili PUT (podrazumevano: DELETE)

Pošto svi zahtevi sadrže ID, Create i Update operacije su idempotentne po podrazumevanoj vrednosti (PUT). Ponavljanje istog Create ili Update zahteva ne bi trebalo da kreira duplikate objekata na vašoj strani.

#### Zaglavlja zahteva

Svaki webhook zahtev uključuje sledeća zaglavlja:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (sekunde) kada je zahtev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Sigurnost i API tokeni](/guides/webhooks/webhooks-api-tokens) za informacije o verifikaciji HMAC potpisa.