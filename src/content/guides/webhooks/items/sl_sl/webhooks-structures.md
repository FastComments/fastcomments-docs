Edina struktura, poslana prek webhookov, je objekt WebhookComment, opisan v TypeScriptu spodaj.

#### Struktura objekta WebhookComment

##### Struktura dogodka "Create"
Telo zahteve za dogodek "create" je objekt WebhookComment.

##### Struktura dogodka "Update"
Telo zahteve za dogodek "update" je objekt WebhookComment.

##### Struktura dogodka "Delete"
Telo zahteve za dogodek "delete" je objekt WebhookComment.

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'Objekt WebhookComment'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** Id komentarja. **/
    id: string
    /** Id ali URL, ki identificira nit komentarjev. Normalizirano. **/
    urlId: string
    /** URL, ki kaže, kje je bil komentar objavljen. **/
    url?: string
    /** Id uporabnika, ki je pustil komentar. Če SSO, s predpono id najemnika. **/
    userId?: string
    /** Email uporabnika, ki je pustil komentar. **/
    commenterEmail?: string
    /** Ime uporabnika, ki se prikaže v pripomočku za komentarje. Pri SSO je lahko displayName. **/
    commenterName: string
    /** Surovo besedilo komentarja. **/
    comment: string
    /** Besedilo komentarja po razčlenitvi. **/
    commentHTML: string
    /** Zunanji id komentarja. **/
    externalId?: string
    /** Id nadrejenega komentarja. **/
    parentId?: string | null
    /** UTC datum, ko je bil komentar objavljen. **/
    date: UTC_ISO_DateString
    /** Združena karma (glasovi gor - dol). **/
    votes: number
    votesUp: number
    votesDown: number
    /** True, če je bil uporabnik prijavljen, ko je komentiral, ali je bil komentar preverjen, ali če je uporabnik preveril svojo sejo, ko je bil komentar objavljen. **/
    verified: boolean
    /** Datum, ko je bil komentar preverjen. **/
    verifiedDate?: number
    /** Če je moderator označil komentar kot pregledan. **/
    reviewed: boolean
    /** Lokacija ali base64-kodirana predstavitev avatarja. Bo base64 samo, če je bila ta vrednost posredovana s SSO. **/
    avatarSrc?: string
    /** Ali je bil komentar ročno ali samodejno označen kot neželena pošta? **/
    isSpam: boolean
    /** Ali je bil komentar samodejno označen kot neželena pošta? **/
    aiDeterminedSpam: boolean
    /** Ali so v komentarju slike? **/
    hasImages: boolean
    /** Številka strani, na kateri je komentar za smer sortiranja "Most Relevant". **/
    pageNumber: number
    /** Številka strani, na kateri je komentar za smer sortiranja "Oldest First". **/
    pageNumberOF: number
    /** Številka strani, na kateri je komentar za smer sortiranja "Newest First". **/
    pageNumberNF: number
    /** Ali je bil komentar odobren samodejno ali ročno? **/
    approved: boolean
    /** Koda lokalizacije (format: en_us) uporabnika, ko je bil komentar napisan. **/
    locale: string
    /** @omenitve, napisane v komentarju, ki so bile uspešno razčlenjene. **/
    mentions?: CommentUserMention[]
    /** Domena, iz katere izvira komentar. **/
    domain?: string
    /** Neobvezen seznam id-jev skupin moderiranja, povezanih s tem komentarjem. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

Ko so uporabniki omenjeni v komentarju, se informacije shranijo v seznam, imenovan `mentions`. Vsak objekt v tem seznamu ima naslednjo strukturo.

[inline-code-attrs-start title = 'Objekt omenitev Webhook'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id uporabnika. Za SSO uporabnike bo imel predpono id najemnika. **/
    id: string
    /** Končno besedilo @mention oznake, vključno z znakom @. **/
    tag: string
    /** Izvirno besedilo @mention oznake, vključno z znakom @. **/
    rawTag: string
    /** Kakšna vrsta uporabnika je bila omenjena. user = račun FastComments.com. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Če se uporabnik odjavi od obvestil, bo to še vedno nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP metode

V nadzorni plošči lahko nastavite HTTP metodo za vsako vrsto dogodka webhook:

- **Dogodek "Create"**: POST ali PUT (privzeto: PUT)
- **Dogodek "Update"**: POST ali PUT (privzeto: PUT)
- **Dogodek "Delete"**: DELETE, POST ali PUT (privzeto: DELETE)

Ker vsi zahtevki vsebujejo ID, sta operaciji Create in Update privzeto idempotentni (PUT). Ponavljanje iste zahteve Create ali Update ne bi smelo ustvariti podvojenih objektov na vaši strani.

#### Glave zahtev

Vsak webhook zahtevek vključuje naslednje glave:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Vaš API skrivni ključ |
| `X-FastComments-Timestamp` | Unix časovni žig (v sekundah), ko je bil zahtevek podpisan |
| `X-FastComments-Signature` | HMAC-SHA256 podpis (`sha256=<hex>`) |

Oglejte si [Varnost in API žetoni](/guides/webhooks/webhooks-api-tokens) za informacije o preverjanju HMAC podpisa.

---