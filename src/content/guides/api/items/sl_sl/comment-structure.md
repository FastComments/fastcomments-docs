Objekt `Comment` predstavlja komentar, ki ga je pustil uporabnik.

Razmerje med starševskimi in otroškimi komentarji je določeno prek `parentId`.

Struktura objekta Comment je naslednja:

[inline-code-attrs-start title = 'Struktura komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SAMO ZA BRANJE: Nastavite na true, če je sistem za zaznavanje neželene pošte označil komentar kot spam. **/
    aiDeterminedSpam?: boolean
    /** Ali je komentar odobren za prikaz. Pri shranjevanju komentarja nastavite na true, drugače bo skrit. **/
    approved?: boolean
    /** Avatar uporabnika. **/
    avatarSrc?: string
    /** Otroški komentarji. Ni vedno napolnjeno v vseh scenarijih. Uporablja se, ko je preko API nastavljen asTree na true. **/
    children: Comment[]
    /** Izvirni komentar avtorja. **/
    comment: string
    /** SAMO ZA BRANJE: Komentar avtorja pretvorjen v HTML. **/
    commentHTML?: string
    /** E-pošta avtorja. Obvezno, če so anonimni komentarji onemogočeni. **/
    commenterEmail?: string
    /** Povezava avtorja (na primer njihov blog). **/
    commenterLink?: string
    /** Ime avtorja komentarja. Vedno obvezno. Če ni na voljo, nastavite nekaj, na primer "Anonimno". **/
    commenterName: string
    /** Datum, ko je bil komentar oddan, v UTC epohi. **/
    date: number
    /** Prikazna oznaka za komentar - na primer "Admin", "Moderator", ali nekaj kot "VIP User". **/
    displayLabel?: string
    /** Domena, na kateri je bil komentar objavljen. **/
    domain?: string
    /** SAMO ZA BRANJE: Številokrat, ko je bil komentar prijavljen. **/
    flagCount?: number
    /** Hashtagi, zapisani v komentarju, ki so bili uspešno razčlenjeni. Lahko tudi ročno dodate hashtage za poizvedovanje, vendar se ti ne bodo samodejno prikazali v besedilu komentarja. **/
    hashTags?: CommentHashTag[]
    /** SAMO ZA BRANJE: Ali komentar vsebuje slike? **/
    hasImages?: boolean
    /** SAMO ZA BRANJE: Ali komentar vsebuje povezave? **/
    hasLinks?: boolean
    /** SAMO ZA BRANJE: Unikatni id komentarja. **/
    id: string
    /** Samo ob ustvarjanju! Za shranjevanje se zabeleži kot hash. **/
    ip?: string
    /** SAMO ZA BRANJE: Ali je trenutni uporabnik blokiral uporabnika, ki je napisal ta komentar? **/
    isBlocked?: boolean
    /** SAMO ZA BRANJE: Ali je komentar napisal administrator? Samodejno nastavljeno na podlagi userId. **/
    isByAdmin?: boolean
    /** SAMO ZA BRANJE: Ali je komentar napisal moderator? Samodejno nastavljeno na podlagi userId. **/
    isByModerator?: boolean
    /** Nastavite na true, če je bil komentar mehko izbrisan (zaradi druge konfiguracije je bilo treba pustiti nadomestni zapis). **/
    isDeleted?: boolean
    /** Nastavite na true, če je bil račun uporabnika izbrisan in je bilo treba komentar obdržati. **/
    isDeletedUser?: boolean
    /** SAMO ZA BRANJE: Ali ga je trenutno prijavljeni uporabnik (contextUserId) označil? **/
    isFlagged?: boolean
    /** Ali je komentar pripet? **/
    isPinned?: boolean
    /** Ali je komentar zaklenjen? Če je true, nihče (tudi moderatorji) nanj ne more odgovoriti, urejati ali izbrisati, dokler ni odklenjen. **/
    isLocked?: boolean
    /** Ali je komentar spam? **/
    isSpam?: boolean
    /** SAMO ZA BRANJE: Ali je komentar ocenjen negativno za trenutnega uporabnika (contextUserId)? **/
    isVotedDown?: boolean
    /** SAMO ZA BRANJE: Ali je komentar ocenjen pozitivno za trenutnega uporabnika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokalizacija (locale), v kateri je komentar. Če ni navedena, bo izpeljana iz HTTP glave Accept-Language. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SAMO ZA BRANJE: @omembe, zapisane v komentarju, ki so bile uspešno razčlenjene. **/
    mentions?: CommentUserMention[]
    /** Dodatni metapodatki, povezani s komentarjem. **/
    meta?: Record<string, string | number | boolean>
    /** Neobvezni seznam id-jev skupin moderiranja, povezanih s tem komentarjem. **/
    moderationGroupIds?: string[]|null
    /** SAMO ZA BRANJE: ID objekta glasovanja, ki ustreza glasu trenutnega uporabnika (contextUserId) za ta komentar. **/
    myVoteId?: string
    /** Ali so bila za ta komentar poslana obvestila avtorjem komentarjev. Če želite preprečiti pošiljanje obvestil pri uvozih, nastavite to na true. **/
    notificationSentForParent?: boolean
    /** Ali so bila za ta komentar poslana obvestila uporabnikom najemnika (tenant). Če želite preprečiti pošiljanje obvestil pri uvozih, nastavite to na true. **/
    notificationSentForParentTenant?: boolean
    /** Naslov strani, na kateri je bil ta komentar. **/
    pageTitle?: string
    /** Če odgovarjamo na komentar, je to ID komentarja, na katerega odgovarjamo. **/
    parentId?: string|null
    /** Ali je komentar označen kot pregledan. **/
    reviewed: boolean
    /** ID najemnika (tenant), kateremu komentar pripada. **/
    tenantId: string
    /** Uporabnik, ki je napisal komentar. Ustvari se samodejno ob shranjevanju komentarja z imenom/e-pošto. **/
    userId?: string|null
    /** URL lokacije, kjer je komentar viden, na primer objava na blogu. **/
    url: string
    /** "Očiščena" različica urlId, ki ste ga poslali. Pri shranjevanju določite to polje, ob vračanju komentarja pa bo to polje "očiščeno" in vaša izvirna vrednost premaknjena v "urlIdRaw". **/
    urlId: string
    /** SAMO ZA BRANJE: Izvirni urlId, ki ste ga poslali. **/
    urlIdRaw?: string
    /** Ali sta uporabnik in ta komentar preverjena? **/
    verified: boolean
    /** Število pozitivnih glasov. **/
    votesUp?: number
    /** Število negativnih glasov. **/
    votesDown?: number
    /** "Karma" komentarja (= pozitivni glasovi - negativni glasovi). **/
    votes?: number
}
[inline-code-end]

Nekatera od teh polj so označena kot `READONLY` - vrne jih API, vendar jih ni mogoče nastaviti.

### Struktura besedila komentarja

Komentarji so napisani v FastComments različici markdowna, ki je navaden markdown plus tradicionalne oznake v stilu `bbcode` za slike, na primer `[img]path[/img]`.

Besedilo se shranjuje v dveh poljih. Besedilo, ki ga je vnesel uporabnik, se shrani nespremenjeno v polju `comment`. To se izriše in shrani v polju `commentHTML`.

Dovoljene HTML oznake so `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Priporočamo, da prikažete HTML, saj gre za zelo majhen podnabor HTML, zato je izdelava rendererja precej enostavna. Na primer, za to obstaja več knjižnic za React Native in Flutter.

Lahko se odločite prikazati ne-normalizirano vrednost polja `comment`. [Primer parserja je tukaj.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primer parserja je mogoče prilagoditi tudi za delo z HTML in pretvorbo HTML oznak v pričakovane elemente za prikaz na vaši platformi. 

### Označevanje

Ko so uporabniki označeni v komentarju, se informacije shranijo v seznam z imenom `mentions`. Vsak objekt v tem seznamu
ima naslednjo strukturo.

[inline-code-attrs-start title = 'Objekt omemb komentarja'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID uporabnika. Pri SSO uporabnikih bo temu predhodil vaš tenant id. **/
    id: string
    /** Končno besedilo @omenjevalne oznake, vključno z znakom @. **/
    tag: string
    /** Izvirno besedilo @omenjevalne oznake, vključno z znakom @. **/
    rawTag: string
    /** Kakšne vrste uporabnik je bil označen. user = račun na FastComments.com. sso = SSO uporabnik. **/
    type: 'user'|'sso'
    /** Če se uporabnik odjavi od obvestil, bo to kljub temu nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

### Hashtagi

Ko so hashtagi uporabljeni in uspešno razčlenjeni, se informacije shranijo v seznam z imenom `hashTags`. Vsak objekt v tem seznamu
ima naslednjo strukturo. Hashtage je mogoče tudi ročno dodati v polje `hashTags` komentarja za poizvedovanje, če je `retain` nastavljen.

[inline-code-attrs-start title = 'Objekt hashtagov komentarja'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ID hashtaga. **/
    id: string
    /** Končno besedilo #hashtag oznake, vključno z znakom #. **/
    tag: string
    /** Če je hashtag povezan z lastnim URL-jem, bo to definirano. **/
    url?: string
    /** Če naj hashtag obdržimo, tudi če ne obstaja v besedilu komentarja, ko je komentar posodobljen. Uporabno za označevanje komentarjev brez spreminjanja besedila komentarja. **/
    retain?: boolean
}
[inline-code-end]

---