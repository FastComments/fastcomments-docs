Jedina struktura koja se šalje preko webhook-ova je objekat WebhookComment, prikazan u TypeScript-u ispod.

#### Struktura objekta WebhookComment

##### The "Create" Event Structure
Tijelo zahtjeva događaja "create" je objekat WebhookComment.

##### The "Update" Event Structure
Tijelo zahtjeva događaja "update" je objekat WebhookComment.

##### The "Delete" Event Structure
Tijelo zahtjeva događaja "delete" je objekat WebhookComment.

    Promjena od 14. novembra 2023.
    Ranije je tijelo zahtjeva za događaj "delete" sadržavalo samo comment id. Sada sadrži puni komentar u trenutku brisanja.


[inline-code-attrs-start title = 'Objekat WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentara. **/
    id: string
    /** Id ili URL koji identifikuje nit komentara. Normalizovano. **/
    urlId: string
    /** URL koji pokazuje gdje je komentar ostavljen. **/
    url?: string
    /** Id korisnika koji je ostavio komentar. Ako je SSO, prefiksovano sa tenant id. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Sa SSO, može biti displayName. **/
    commenterName: string
    /** Neobrađeni tekst komentara. **/
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
    /** True ako je korisnik bio prijavljen kada je komentarisao, ili su verificirali komentar, ili ako su verificirali svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verificiran. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija, ili base64 enkodovanje, avatara. Biće base64 samo ako je ta vrijednost poslana sa SSO. **/
    avatarSrc?: string
    /** Da li je komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li postoje slike u komentaru? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za smjer sortiranja "Newest First". **/
    pageNumberNF: number
    /** Da li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kôd lokalizacije (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** Domen odakle je komentar. **/
    domain?: string
    /** Opcionalna lista moderation group ids povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi nazvanoj `mentions`. Svaki objekat u toj listi
ima sljedeću strukturu.

[inline-code-attrs-start title = 'Objekat Webhook pomena'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id korisnika. Za SSO korisnike, biće prefiksovano vašim tenant id. **/
    id: string
    /** Konačni tekst @mention taga, uključujući @ simbol. **/
    tag: string
    /** Originalni tekst @mention taga, uključujući @ simbol. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP Methods Used

**Create and Update both use HTTP PUT and not POST!**

Pošto svi naši zahtjevi sadrže ID, ponavljanje istog Create ili Update zahtjeva ne bi trebalo da kreira nove objekte na vašoj strani.

To znači da su ovi pozivi idempotentni i trebaju biti PUT događaji u skladu sa HTTP specifikacijom.

---