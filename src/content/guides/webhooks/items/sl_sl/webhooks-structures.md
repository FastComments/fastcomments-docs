Edina struktura, poslana prek webhookov, je objekt WebhookComment, opisan v TypeScriptu spodaj.

#### Struktura objekta WebhookComment

##### Struktura dogodka "create"
Vsebina zahteve dogodka "create" je objekt WebhookComment.

##### Struktura dogodka "update"
Vsebina zahteve dogodka "update" je objekt WebhookComment.

##### Struktura dogodka "delete"
Vsebina zahteve dogodka "delete" je objekt WebhookComment.

    Sprememba z dne 14. novembra 2023
    Prej je vsebina zahteve dogodka "delete" vsebovala samo id komentarja. Zdaj vsebuje celoten komentar v času brisanja.


[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentarja. **/
    id: string
    /** ID ali URL, ki identificira nit komentarjev. Normalizirano. **/
    urlId: string
    /** URL, ki kaže, kje je bil komentar objavljen. **/
    url?: string
    /** ID uporabnika, ki je pustil komentar. Če gre za SSO, je predponjen z tenant id. **/
    userId?: string
    /** E-pošta uporabnika, ki je pustil komentar. **/
    commenterEmail?: string
    /** Ime uporabnika, ki se prikaže v vtičniku za komentarje. Pri SSO je lahko displayName. **/
    commenterName: string
    /** Neobdelano besedilo komentarja. **/
    comment: string
    /** Besedilo komentarja po parsiranju. **/
    commentHTML: string
    /** Zunanji ID komentarja. **/
    externalId?: string
    /** ID starševskega komentarja. **/
    parentId?: string | null
    /** UTC datum, ko je bil komentar objavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirana karma (up - down) glasov. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True če je bil uporabnik prijavljen, ko je komentiral, ali je preveril komentar, ali je preveril svojo sejo, ko je bil komentar oddan. **/
    verified: boolean
    /** Datum, ko je bil komentar preverjen. **/
    verifiedDate?: number
    /** Če je moderator označil komentar kot pregledan. **/
    reviewed: boolean
    /** Lokacija ali base64 kodiranje avatarja. Bo base64 le, če je bila ta vrednost posredovana pri SSO. **/
    avatarSrc?: string
    /** Ali je bil komentar ročno ali samodejno označen kot spam? **/
    isSpam: boolean
    /** Ali je bil komentar samodejno označen kot spam? **/
    aiDeterminedSpam: boolean
    /** Ali so v komentarju slike? **/
    hasImages: boolean
    /** Številka strani, na kateri se komentar nahaja za razvrščanje "Most Relevant". **/
    pageNumber: number
    /** Številka strani, na kateri se komentar nahaja za razvrščanje "Oldest First". **/
    pageNumberOF: number
    /** Številka strani, na kateri se komentar nahaja za razvrščanje "Newest First". **/
    pageNumberNF: number
    /** Ali je bil komentar odobren samodejno ali ročno? **/
    approved: boolean
    /** Koda lokalizacije (format: en_us) uporabnika ob pisanju komentarja. **/
    locale: string
    /** @mentions zapisane v komentarju, ki so bile uspešno parsirane. **/
    mentions?: CommentUserMention[]
    /** Domena, iz katere je komentar. **/
    domain?: string
    /** Izbirni seznam ID-jev skupin moderiranja, povezanih s tem komentarjem. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Ko so uporabniki označeni v komentarju, se informacije shranijo v seznamu z imenom `mentions`. Vsak objekt v tem seznamu ima naslednjo strukturo.

[inline-code-attrs-start title = 'Objekt omemb webhooka'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID uporabnika. Za SSO uporabnike bo imel predpono vaš tenant id. **/
    id: string
    /** Končno besedilo @mention oznake, vključno s simbolom @. **/
    tag: string
    /** Izvirno besedilo @mention oznake, vključno s simbolom @. **/
    rawTag: string
    /** Kakšna vrsta uporabnika je bila omenjena. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Če se uporabnik odjavi od obvestil, bo to še vedno nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### Uporabljene HTTP metode

**Create and Update both use HTTP PUT and not POST!**

Ker vse naše zahteve vsebujejo ID, ponavljanje iste zahteve Create ali Update ne bi smelo ustvariti novih objektov na vaši strani.

To pomeni, da so ti klici idempotentni in bi morali biti PUT dogodki v skladu s HTTP specifikacijo.