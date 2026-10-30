Edina struktura, poslana prek webhookov, je objekt **WebhookComment**, prikazan v TypeScript spodaj.

#### Struktura objekta WebhookComment

##### Struktura dogodka "Create"
Telo zahteve dogodka **create** je objekt **WebhookComment**.

##### Struktura dogodka "Update"
Telo zahteve dogodka **update** je objekt **WebhookComment**.

##### Struktura dogodka "Delete"
Telo zahteve dogodka **delete** je objekt **WebhookComment**.

Sprememba od 14. novembra 2023  
Prej je telo zahteve dogodka **delete** vsebovalo le ID komentarja. Zdaj vsebuje celoten komentar v času brisanja.

Vsak ključ je v telesu vedno prisoten. Ko komentar nima vrednosti za polje, telo nosi `null`  
(ali `false` za logične vrednosti in `[]` za sezname), tako da se struktura dostave nikoli ne spreminja med komentarji.

[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** ID komentarja. **/
    id: string
    /** ID ali URL, ki identificira nit komentarjev. Normalizirano. **/
    urlId: string
    /** URL, ki kaže na mesto, kjer je bil komentar objavljen. **/
    url: string | null
    /** ID uporabnika, ki je objavil komentar. Če SSO, je predponjen z ID najemnika. **/
    userId: string | null
    /** E‑mail uporabnika, ki je objavil komentar. **/
    commenterEmail: string | null
    /** Ime uporabnika, ki se prikaže v pripomočku za komentarje. Pri SSO je lahko displayName. **/
    commenterName: string
    /** Surovo besedilo komentarja. **/
    comment: string
    /** Besedilo komentarja po obdelavi. **/
    commentHTML: string
    /** Zunanji ID komentarja. **/
    externalId: string | null
    /** ID nadrejenega komentarja. **/
    parentId: string | null
    /** Datum v UTC, ko je bil komentar objavljen. **/
    date: UTC_ISO_DateString
    /** Kombinirano karma (glasovi gor - dol). **/
    votes: number
    votesUp: number
    votesDown: number
    /** Resnično, če je bil uporabnik prijavljen, ko je komentiral, ali je potrdil komentar, ali je potrdil sejo, ko je bil komentar objavljen. **/
    verified: boolean
    /** Datum v UTC, ko je bil komentar potrjen. **/
    verifiedDate: UTC_ISO_DateString | null
    /** Če je moderator označil komentar kot pregledan. **/
    reviewed: boolean
    /** Lokacija ali base64 kodiranje avatarja. Base64 bo le, če je bila ta vrednost posredovana prek SSO. **/
    avatarSrc: string | null
    /** Ali je bil komentar ročno ali samodejno označen kot neželen (spam)? **/
    isSpam: boolean
    /** Ali je bil komentar samodejno označen kot spam? **/
    aiDeterminedSpam: boolean
    /** Ali komentar vsebuje slike? **/
    hasImages: boolean
    /** Številka strani, na kateri je komentar pri razvrščanju po "Najbolj relevantnih". **/
    pageNumber: number | null
    /** Številka strani, na kateri je komentar pri razvrščanju po "Starejši najprej". **/
    pageNumberOF: number | null
    /** Številka strani, na kateri je komentar pri razvrščanju po "Novejši najprej". **/
    pageNumberNF: number | null
    /** Ali je bil komentar odobren samodejno ali ročno? **/
    approved: boolean
    /** Koda jezika (format: en_us) uporabnika, ko je bil komentar napisan. **/
    locale: string | null
    /** @omenjave, zapisane v komentarju, ki so bile uspešno razčlenjene. Prazno, ko jih ni. **/
    mentions: CommentUserMention[]
    /** Domena, iz katere je komentar. **/
    domain: string | null
    /** ID-ji moderacijskih skupin, povezanih s tem komentarjem. Prazno, ko jih ni. **/
    moderationGroupIds: string[]
    /** Značke, prikazane poleg imena komentatorja pri tem komentarju, v času nastanka dogodka. Prazno, ko jih ni. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

Ko so uporabniki v komentarju označeni, se informacije shranijo v seznamu z imenom `mentions`. Vsak objekt v tem seznamu ima naslednjo strukturo.

[inline-code-attrs-start title = 'Objekt omemb Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID uporabnika. Za SSO uporabnike bo predponjen z ID najemnika. **/
    id: string
    /** Končni besedilni niz @omenjave, vključno s simbolom @. **/
    tag: string
    /** Izvirni besedilni niz @omenjave, vključno s simbolom @. **/
    rawTag: string
    /** Vrsta označenega uporabnika. user = račun FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Če se uporabnik odjavi od obvestil, bo to še vedno nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

Vsak vnos v seznamu `badges` ima naslednjo strukturo. Za vrste značk si oglejte [vodič po značkah](/guide-badges.html).

[inline-code-attrs-start title = 'Objekt značke Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** ID značke, dodeljene temu uporabniku. **/
    id: string
    /** Vrsta značke. **/
    type: number
    /** Opis značke, prikazan ob prehodu z miško. **/
    description: string
    /** Besedilo, prikazano na znački. **/
    displayLabel?: string | null
    /** Slika, prikazana za značko, če jo uporablja. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** Dodatni CSS razred, uporabljen za značko. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP metode

Metodo HTTP za vsak tip webhook dogodka lahko nastavite v skrbniškem vmesniku:

- **Dogodek Create**: POST ali PUT (privzeto: PUT)
- **Dogodek Update**: POST ali PUT (privzeto: PUT)
- **Dogodek Delete**: DELETE, POST ali PUT (privzeto: DELETE)

Ker vse zahteve vsebujejo ID, sta operaciji Create in Update privzeto idempotentni (PUT). Ponovitev iste zahteve Create ali Update ne bi smela ustvariti podvojenih objektov na vaši strani.

#### Glave zahteve

Vsaka zahteva webhooka vključuje naslednje glave:

| Glava | Opis |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API skrivni ključ |
| `X-FastComments-Timestamp` | Unix časovni žig (sekunde), ko je bila zahteva podpisana |
| `X-FastComments-Signature` | HMAC-SHA256 podpis (`sha256=<hex>`) |

Za informacije o preverjanju HMAC podpisa si oglejte [Varnost in API žetoni](/guide-webhooks.html#webhooks-api-tokens).