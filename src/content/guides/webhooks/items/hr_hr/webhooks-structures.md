Jedina struktura poslana putem webhookova je objekt WebhookComment, prikazan u TypeScriptu u nastavku.

#### Struktura objekta WebhookComment

##### Struktura događaja "Create"
Tijelo zahtjeva za događaj "create" je objekt WebhookComment.

##### Struktura događaja "Update"
Tijelo zahtjeva za događaj "update" je objekt WebhookComment.

##### Struktura događaja "Delete"
Tijelo zahtjeva za događaj "delete" je objekt WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentara. **/
    id: string
    /** ID ili URL koji identificira nit komentara. Normalizirano. **/
    urlId: string
    /** URL koji pokazuje gdje je komentar ostavljen. **/
    url?: string
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiksan s tenant ID-jem. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Kod SSO, može biti displayName. **/
    commenterName: string
    /** Izvorni tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Vanjski ID komentara. **/
    externalId?: string
    /** ID nadređenog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirani karma (up - down) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio prijavljen kada je komentirao, ili je verificirao komentar, ili je verificirao svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verificiran. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija, ili base64 kodiranje, avatara. Bit će base64 samo ako je ta vrijednost poslana sa SSO. **/
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
    /** Kod lokalizacije (format: en_us) korisnika u trenutku pisanja komentara. **/
    locale: string
    /** @mentions napisani u komentaru koji su uspješno parsirani. **/
    mentions?: CommentUserMention[]
    /** Domena iz koje potječe komentar. **/
    domain?: string
    /** Opcionalni popis ID-jeva moderacijskih grupa pridruženih ovom komentaru. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kada su korisnici označeni u komentaru, informacije se spremaju u listu nazvanu `mentions`. Svaki objekt u toj listi ima sljedeću strukturu.

[inline-code-attrs-start title = 'Objekt spominjanja u webhooku'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, imat će prefiks s vašim tenant ID-jem. **/
    id: string
    /** Konačni tekst @mention oznake, uključujući simbol @. **/
    tag: string
    /** Izvorni tekst @mention oznake, uključujući simbol @. **/
    rawTag: string
    /** Koja je vrsta korisnika bila označena. user = FastComments.com račun. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od obavijesti, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### Korištene HTTP metode

**Create i Update oba koriste HTTP PUT, a ne POST!**

Budući da svi naši zahtjevi sadrže ID, ponavljanje istog Create ili Update zahtjeva ne bi trebalo stvoriti nove objekte na vašoj strani.

To znači da su ti pozivi idempotentni i trebali bi biti PUT događaji prema HTTP specifikaciji.