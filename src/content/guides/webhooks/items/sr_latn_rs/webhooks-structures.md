The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### Struktura objekta WebhookComment

##### Struktura događaja „Create“
The "create" event request body is a WebhookComment object.

##### Struktura događaja „Update“
The "update" event request body is a WebhookComment object.

##### Struktura događaja „Delete“
The "delete" event request body is a WebhookComment object.

    Izmena od 14. novembra 2023.
    Prethodno je telo zahteva za događaj „delete“ sadržalo samo ID komentara. Sada sadrži ceo komentar u trenutku brisanja.

Svaki ključ je uvek prisutan u telu. Kada komentar nema vrednost za neko polje, telo nosi `null` (ili `false` za logičke vrednosti i `[]` za liste), tako da oblik isporuke nikada ne varira od jednog komentara do drugog.

[inline-code-attrs-start title = 'Objekat WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentara. **/
    id: string
    /** ID ili URL koji identifikuje nit komentara. Normalizovano. **/
    urlId: string
    /** URL koji pokazuje gde je komentar ostavljen. **/
    url: string | null
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiks je ID zakupca. **/
    userId: string | null
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail: string | null
    /** Ime korisnika koje se prikazuje u vidžetu za komentar. Sa SSO, može biti displayName. **/
    commenterName: string
    /** Sirov tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Eksterni ID komentara. **/
    externalId: string | null
    /** ID nadređenog komentara. **/
    parentId: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinovani karma (glasovi gore - dole). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Istina ako je korisnik bio prijavljen kada je komentarisao, ili je verifikovao komentar, ili je verifikovao sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** UTC datum kada je komentar verifikovan. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija ili base64 kodiranje avatara. Biće base64 samo ako je to vrednost prosleđena sa SSO. **/
    avatarSrc: string | null
    /** Da li je komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li u komentaru postoje slike? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za sortiranje „Najrelevantnije“. **/
    pageNumber: number | null
    /** Broj stranice na kojoj se komentar nalazi za sortiranje „Najstariji prvi“. **/
    pageNumberOF: number | null
    /** Broj stranice na kojoj se komentar nalazi za sortiranje „Najnoviji prvi“. **/
    pageNumberNF: number | null
    /** Da li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod lokalizacije (format: en_us) korisnika kada je komentar napisan. **/
    locale: string | null
    /** @pomeni napisani u komentaru koji su uspešno parsirani. Prazno kada ih nema. **/
    mentions: CommentUserMention[]
    /** Domen iz kojeg je komentar. **/
    domain: string | null
    /** ID‑ovi grupa moderacije povezani sa ovim komentarom. Prazno kada ih nema. **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Objekat Webhook Mentions'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, biće prefiksiran ID vašeg zakupca. **/
    id: string
    /** Konačni tekst @mention taga, uključujući @ simbol. **/
    tag: string
    /** Originalni tekst @mention taga, uključujući @ simbol. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od obaveštenja, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Metode

Možete konfigurisati HTTP metodu za svaki tip webhook događaja u administratorskom panelu:

- **Create događaj**: POST ili PUT (podrazumevano: PUT)
- **Update događaj**: POST ili PUT (podrazumevano: PUT)
- **Delete događaj**: DELETE, POST ili PUT (podrazumevano: DELETE)

Pošto svi zahtevi sadrže ID, operacije Create i Update su po podrazumevanju idempotentne (PUT). Ponovljeni isti Create ili Update zahtev ne bi trebalo da kreira duple objekte na vašoj strani.

#### Zaglavlja zahteva

Svaki webhook zahtev uključuje sledeća zaglavlja:

| Zaglavlje | Opis |
|-----------|------|
| `Content-Type` | `application/json` |
| `token` | Vaša API tajna |
| `X-FastComments-Timestamp` | Unix vremenski žig (sekunde) kada je zahtev potpisan |
| `X-FastComments-Signature` | HMAC-SHA256 potpis (`sha256=<hex>`) |

Pogledajte [Bezbednost i API tokeni](/guide-webhooks.html#webhooks-api-tokens) za informacije o verifikaciji HMAC potpisa.