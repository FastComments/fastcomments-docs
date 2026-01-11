Jedina struktura koja se šalje putem webhook-ova je objekat WebhookComment, prikazan u TypeScript-u ispod.

#### Struktura objekta WebhookComment

##### Struktura događaja "Create"
Tijelo zahtjeva za događaj "create" je objekat WebhookComment.

##### Struktura događaja "Update"
Tijelo zahtjeva za događaj "update" je objekat WebhookComment.

##### Struktura događaja "Delete"
Tijelo zahtjeva za događaj "delete" je objekat WebhookComment.

    Promjena od 14. novembra 2023.
    Ranije je tijelo zahtjeva za događaj "delete" sadržavalo samo id komentara. Sada sadrži puni komentar onakav kakav je bio u vrijeme brisanja.


[inline-code-attrs-start title = 'Objekat WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentara. **/
    id: string
    /** ID ili URL koji identificira nit komentara. Normalizovano. **/
    urlId: string
    /** URL koji pokazuje gdje je komentar ostavljen. **/
    url?: string
    /** ID korisnika koji je ostavio komentar. Ako je SSO, prefiksiran ID-jem tenanta. **/
    userId?: string
    /** Email korisnika koji je ostavio komentar. **/
    commenterEmail?: string
    /** Ime korisnika koje se prikazuje u widgetu komentara. Kod SSO može biti displayName. **/
    commenterName: string
    /** Neobrađeni tekst komentara. **/
    comment: string
    /** Tekst komentara nakon parsiranja. **/
    commentHTML: string
    /** Eksterni ID komentara. **/
    externalId?: string
    /** ID roditeljskog komentara. **/
    parentId?: string | null
    /** UTC datum kada je komentar ostavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirani karma (za - protiv) glasova. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True ako je korisnik bio prijavljen kada je komentarisao, ili je verificirao komentar, ili je verificirao svoju sesiju kada je komentar ostavljen. **/
    verified: boolean
    /** Datum kada je komentar verificiran. **/
    verifiedDate?: number
    /** Ako je moderator označio komentar kao pregledan. **/
    reviewed: boolean
    /** Lokacija, ili base64 enkodiranje avatara. Biće base64 samo ako je ta vrijednost proslijeđena uz SSO. **/
    avatarSrc?: string
    /** Da li je komentar ručno ili automatski označen kao spam? **/
    isSpam: boolean
    /** Da li je komentar automatski označen kao spam? **/
    aiDeterminedSpam: boolean
    /** Da li se u komentaru nalaze slike? **/
    hasImages: boolean
    /** Broj stranice na kojoj se komentar nalazi za način sortiranja "Most Relevant". **/
    pageNumber: number
    /** Broj stranice na kojoj se komentar nalazi za način sortiranja "Oldest First". **/
    pageNumberOF: number
    /** Broj stranice na kojoj se komentar nalazi za način sortiranja "Newest First". **/
    pageNumberNF: number
    /** Da li je komentar odobren automatski ili ručno? **/
    approved: boolean
    /** Kod lokalizacije (format: en_us) korisnika kada je komentar napisan. **/
    locale: string
    /** The @mentions written in the comment that were successfully parsed. **/
    mentions?: CommentUserMention[]
    /** Domen iz kojeg potiče komentar. **/
    domain?: string
    /** Opcionalna lista ID-jeva grupa za moderaciju povezanih s ovim komentarom. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Objekat pomena u webhook-u'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

#### Korištene HTTP metode

**Create i Update oba koriste HTTP PUT, a ne POST!**

Pošto svi naši zahtjevi sadrže ID, ponavljanje istog Create ili Update zahtjeva ne bi trebalo da kreira nove objekte na vašoj strani.

To znači da su ovi pozivi idempotentni i trebaju biti PUT događaji u skladu sa HTTP specifikacijom.