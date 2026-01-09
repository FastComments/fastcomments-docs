Objekt `Comment` predstavlja komentar, ki ga je pustil uporabnik.

Razmerje med nadrejenimi in potomskimi komentarji je določeno z `parentId`.

Struktura objekta Comment je naslednja:

[inline-code-attrs-start title = 'Struktura komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Nastavljeno na true, če je protivspamski mehanizem ocenil, da je komentar spam. **/
    aiDeterminedSpam?: boolean
    /** Ali je komentar odobren za prikaz. Ob shranjevanju komentarja nastavljeno na true, sicer bo skrit. **/
    approved?: boolean
    /** Avatar uporabnika. **/
    avatarSrc?: string
    /** Potomski komentarji. V vseh scenarijih niso vedno napolnjeni. Uporablja se, ko je preko API-ja asTree nastavljen na true. **/
    children: Comment[]
    /** Izviren komentar avtorja. **/
    comment: string
    /** READONLY: Komentar avtorja, pretvorjen v HTML. **/
    commentHTML?: string
    /** E-pošta avtorja. Obvezna, če je anonimno komentiranje onemogočeno. **/
    commenterEmail?: string
    /** Povezava avtorja (npr. njihov blog). **/
    commenterLink?: string
    /** Ime avtorja komentarja. Vedno zahtevano. Če ni na voljo, nastavite nekaj, kot je "Anonymous". **/
    commenterName: string
    /** Datum, kdaj je bil komentar objavljen, v UTC epoch obliki. **/
    date: number
    /** "Prikazna oznaka" za komentar - na primer "Admin", "Moderator", ali nekaj kot "VIP User". **/
    displayLabel?: string
    /** Domena, na kateri je bil komentar objavljen. **/
    domain?: string
    /** READONLY: Število krat, ko je bil komentar prijavljen (flag). **/
    flagCount?: number
    /** #hashtag-i zapisani v komentarju, ki so bili uspešno razčlenjeni. Hashtage lahko tudi ročno dodate za poizvedovanje, vendar se ti ne bodo samodejno prikazali v besedilu komentarja. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Ali komentar vsebuje slike? **/
    hasImages?: boolean
    /** READONLY: Ali komentar vsebuje povezave? **/
    hasLinks?: boolean
    /** READONLY: Edinstveni id komentarja. **/
    id: string
    /** Samo ob ustvarjanju! To je zgoščeno (hashed) za shranjevanje. **/
    ip?: string
    /** READONLY: Ali je trenutni uporabnik blokiral uporabnika, ki je napisal ta komentar? **/
    isBlocked?: boolean
    /** READONLY: Ali je komentar avtorjev admin? Samodejno nastavljeno glede na userId. **/
    isByAdmin?: boolean
    /** READONLY: Ali je komentar avtorjev moderator? Samodejno nastavljeno glede na userId. **/
    isByModerator?: boolean
    /** Nastavljeno na true, če je bil komentar mehko izbrisan (mora ostati priponka zaradi določene konfiguracije). **/
    isDeleted?: boolean
    /** Nastavljeno na true, če je bil uporabniški račun izbrisan in je bilo treba komentar obdržati. **/
    isDeletedUser?: boolean
    /** READONLY: Ali je komentar prijavljen (flag) s strani trenutno prijavljenega uporabnika (contextUserId)? **/
    isFlagged?: boolean
    /** Ali je komentar pripet (pinned)? **/
    isPinned?: boolean
    /** Ali je komentar zaklenjen za nove odgovore (moderatorji lahko še vedno odgovarjajo)? **/
    isLocked?: boolean
    /** Ali je komentar spam? **/
    isSpam?: boolean
    /** READONLY: Ali je komentar za trenutnega uporabnika (contextUserId) ocenjeno navzdol? **/
    isVotedDown?: boolean
    /** READONLY: Ali je komentar za trenutnega uporabnika (contextUserId) ocenjeno navzgor? **/
    isVotedUp?: boolean
    /** Jeziki komentarja. Če ni podano, bo izpeljano iz HTTP glave language accept. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: @omembe, zapisane v komentarju, ki so bile uspešno razčlenjene. **/
    mentions?: CommentUserMention[]
    /** Neobvezni metapodatki povezani s komentarjem. **/
    meta?: Record<string, string | number | boolean>
    /** Neobvezni seznam id-jev moderacijskih skupin, povezanih s tem komentarjem. **/
    moderationGroupIds?: string[]|null
    /** READONLY: Id objekta glasovanja, ki ustreza glasu trenutnega uporabnika (contextUserId) za ta komentar. **/
    myVoteId?: string
    /** Ali so bila za ta komentar poslana obvestila komentatorjem. Da preprečite pošiljanje obvestil pri uvozih, nastavite na true. **/
    notificationSentForParent?: boolean
    /** Ali so bila za ta komentar poslana obvestila uporabnikom najemnika. Da preprečite pošiljanje obvestil pri uvozih, nastavite na true. **/
    notificationSentForParentTenant?: boolean
    /** Naslov strani, na kateri je bil komentar. **/
    pageTitle?: string
    /** Če odgovarjamo na komentar, je to ID komentarja, na katerega odgovarjamo. **/
    parentId?: string|null
    /** Ali je komentar označen kot pregledan. **/
    reviewed: boolean
    /** Id najemnika (tenant), kjer komentar pripada. **/
    tenantId: string
    /** Uporabnik, ki je napisal komentar. Ustvarjen samodejno ob shranjevanju komentarja z imenom/e-pošto. **/
    userId?: string|null
    /** URL lokacije, kjer je komentar viden, na primer objava na blogu. **/
    url: string
    /** "Očiščena" različica urlId, ki ste jo poslali. Pri shranjevanju določite to polje, a ko pridobite komentar nazaj, bo to "očiščeno" in vaša originalna vrednost premaknjena v "urlIdRaw". **/
    urlId: string
    /** READONLY: Izvirni urlId, ki ste nam ga poslali. **/
    urlIdRaw?: string
    /** Ali sta uporabnik in ta komentar preverjena? **/
    verified: boolean
    /** Število glasov za. **/
    votesUp?: number
    /** Število glasov proti. **/
    votesDown?: number
    /** "Karma" komentarja (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

Nekatera od teh polj so označena kot `READONLY` - ta so vrnjena s strani API-ja, vendar jih ni mogoče nastaviti.

### Struktura besedila komentarja

Komentarji so napisani v FastComments različici markdowna, kar je preprosto markdown plus tradicionalne `bbcode` sloga oznake za slike, na primer `[img]path[/img]`.

Besedilo se shranjuje v dveh poljih. Besedilo, ki ga je uporabnik vnesel, je shranjeno nespremenjeno v polju `comment`. To se upodobi in shrani v polju `commentHTML`.

Dovoljene HTML oznake so `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Priporočeno je upodabljanje HTML-ja, saj gre za zelo majhen podnabor HTML-ja, zato je izgradnja rendererja precej preprosta. Na voljo je več knjižnic za React Native in Flutter, na primer, ki pri tem pomagajo.

Lahko se odločite tudi za upodabljanje ne-normalizirane vrednosti polja `comment`. [Primer parserja je tukaj.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primer parserja bi lahko prilagodili tudi za delo s HTML-jem in pretvorbo HTML oznak v pričakovane elemente za upodabljanje na vaši platformi. 

### Označevanje

Ko so uporabniki označeni v komentarju, je informacija shranjena v seznamu imenovanem `mentions`. Vsak objekt v tem seznamu ima naslednjo strukturo.

[inline-code-attrs-start title = 'Objekt omemb komentarja'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** Id uporabnika. Pri SSO uporabnikih bo temu predponan vaš tenant id. **/
    id: string
    /** Končni @mention tag besedila, vključno z znakom @. **/
    tag: string
    /** Izvirni @mention tag besedila, vključno z znakom @. **/
    rawTag: string
    /** Kakšen tip uporabnika je bil označen. user = FastComments.com račun. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Če se uporabnik odjavi od obvestil, bo to še vedno nastavljeno na true. **/
    sent: boolean
}
[inline-code-end]

### Hashtagi

Ko so hashtagi uporabljeni in uspešno razčlenjeni, je informacija shranjena v seznamu imenovanem `hashTags`. Vsak objekt v tem seznamu ima naslednjo strukturo. Hashtage je mogoče tudi ročno dodati v polje `hashTags` komentarja za poizvedovanje, če je `retain` nastavljen.

[inline-code-attrs-start title = 'Objekt hash-tagov komentarja'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** Id hashtaga. **/
    id: string
    /** Končni #hashtag tag besedila, vključno z znakom #. **/
    tag: string
    /** Če je hashtag povezan z lastnim URL-jem, bo to definirano. **/
    url?: string
    /** Če naj hashtag obdržimo, tudi če ob posodobitvi komentarja ni prisoten v besedilu. Uporabno za označevanje komentarjev brez spreminjanja besedila komentarja. **/
    retain?: boolean
}
[inline-code-end]

---