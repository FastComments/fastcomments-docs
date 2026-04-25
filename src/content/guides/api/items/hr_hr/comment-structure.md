Objekt `Comment` predstavlja komentar koji je ostavio korisnik.

Odnos između roditeljskih i podređenih komentara definiran je putem `parentId`.

Struktura objekta Comment je sljedeća:

[inline-code-attrs-start title = 'Struktura Comment objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SAMO ZA ČITANJE: Postavljeno na true ako je sustav za otkrivanje spama utvrdio da je komentar spam. **/
    aiDeterminedSpam?: boolean
    /** Je li komentar odobren za prikaz. Postavljeno na true pri spremanju komentara, inače će biti skriven. **/
    approved?: boolean
    /** Avatar korisnika. **/
    avatarSrc?: string
    /** Podređeni komentari. Nisu popunjeni u svim scenarijima. Koristi se kada je asTree postavljeno na true putem API-ja. **/
    children: Comment[]
    /** Izvorni (neobrađeni) komentar autora. **/
    comment: string
    /** SAMO ZA ČITANJE: Komentar autora parsiran u HTML. **/
    commentHTML?: string
    /** Email autora komentara. Obavezno ako je anonimno komentiranje isključeno. **/
    commenterEmail?: string
    /** Link autora komentara (na primjer njihov blog). **/
    commenterLink?: string
    /** Ime autora komentara. Uvijek obavezno. Ako nije dostupno, postavite nešto poput "Anonymous". **/
    commenterName: string
    /** Datum kada je komentar ostavljen, u UTC epoch formatu. **/
    date: number
    /** "Prikazna oznaka" za komentar - na primjer "Admin", "Moderator", ili nešto poput "VIP User". **/
    displayLabel?: string
    /** Domena na kojoj je komentar objavljen. **/
    domain?: string
    /** SAMO ZA ČITANJE: Broj puta koliko je komentar prijavljen. **/
    flagCount?: number
    /** Hashtagovi (#) upisani u komentar koji su uspješno parsirani. Možete također ručno dodati hashtagove za upite, ali oni se neće automatski prikazivati u tekstu komentara. **/
    hashTags?: CommentHashTag[]
    /** SAMO ZA ČITANJE: Sadrži li komentar slike? **/
    hasImages?: boolean
    /** SAMO ZA ČITANJE: Sadrži li komentar linkove? **/
    hasLinks?: boolean
    /** SAMO ZA ČITANJE: Jedinstveni id komentara. **/
    id: string
    /** Samo pri kreiranju! Ovo se hešira za pohranu. **/
    ip?: string
    /** SAMO ZA ČITANJE: Je li trenutni korisnik blokirao korisnika koji je napisao ovaj komentar? **/
    isBlocked?: boolean
    /** SAMO ZA ČITANJE: Je li komentar od administratora? Automatski postavljeno na temelju userId. **/
    isByAdmin?: boolean
    /** SAMO ZA ČITANJE: Je li komentar od moderatora? Automatski postavljeno na temelju userId. **/
    isByModerator?: boolean
    /** Postavite na true ako je komentar meko izbrisan (morao je ostati zamjenski zapis zbog neke druge konfiguracije). **/
    isDeleted?: boolean
    /** Postavite na true ako je korisnički račun izbrisan i komentar je morao biti zadržan. **/
    isDeletedUser?: boolean
    /** SAMO ZA ČITANJE: Je li komentar označen od strane trenutno prijavljenog korisnika (contextUserId)? **/
    isFlagged?: boolean
    /** Je li komentar pričvršćen? **/
    isPinned?: boolean
    /** Je li komentar zaključan? Kada je true, nitko (uključujući moderatore) ne može odgovarati, uređivati ili brisati komentar dok se ne otključa. **/
    isLocked?: boolean
    /** Je li komentar spam? **/
    isSpam?: boolean
    /** SAMO ZA ČITANJE: Je li komentar ocijenjen negativno za trenutnog korisnika (contextUserId)? **/
    isVotedDown?: boolean
    /** SAMO ZA ČITANJE: Je li komentar ocijenjen pozitivno za trenutnog korisnika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokalizacija u kojoj je komentar. Ako nije navedena, izvest će se iz Accept-Language HTTP zaglavlja. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SAMO ZA ČITANJE: @mentions napisane u komentaru koje su uspješno parsirane. **/
    mentions?: CommentUserMention[]
    /** Opcionalni metapodaci povezani s komentarom. **/
    meta?: Record<string, string | number | boolean>
    /** Opcionalna lista id-eva grupa za moderiranje povezanih s ovim komentarom. **/
    moderationGroupIds?: string[]|null
    /** SAMO ZA ČITANJE: ID objekta glasanja koji odgovara glasu trenutnog korisnika (contextUserId) za ovaj komentar. **/
    myVoteId?: string
    /** Jesu li obavijesti poslane za ovaj komentar komentatorima. Da spriječite slanje obavijesti prilikom uvoza, postavite ovo na true. **/
    notificationSentForParent?: boolean
    /** Jesu li obavijesti poslane za ovaj komentar korisnicima tenanta. Da spriječite slanje obavijesti prilikom uvoza, postavite ovo na true. **/
    notificationSentForParentTenant?: boolean
    /** Naslov stranice na kojoj se nalazio ovaj komentar. **/
    pageTitle?: string
    /** Ako odgovaramo na komentar, ovo je ID na koji odgovaramo. **/
    parentId?: string|null
    /** Je li komentar označen kao pregledan. **/
    reviewed: boolean
    /** ID tenanta kojem komentar pripada. **/
    tenantId: string
    /** Korisnik koji je napisao komentar. Kreira se automatski pri spremanju komentara s imenom/emailom. **/
    userId?: string|null
    /** URL lokacije na kojoj je ovaj komentar vidljiv, poput objave na blogu. **/
    url: string
    /** "Očišćena" verzija urlId koju ste nam poslali. Pri spremanju specificirate ovo polje, ali kada dohvatite komentar natrag, ovo će biti "očišćeno" i vaša originalna vrijednost premještena u "urlIdRaw". **/
    urlId: string
    /** SAMO ZA ČITANJE: Originalni urlId koji ste nam poslali. **/
    urlIdRaw?: string
    /** Je li korisnik i ovaj komentar verificiran? **/
    verified: boolean
    /** Broj pozitivnih glasova. **/
    votesUp?: number
    /** Broj negativnih glasova. **/
    votesDown?: number
    /** "Karma" komentara (= broj pozitivnih glasova - broj negativnih glasova). **/
    votes?: number
}
[inline-code-end]

Neka od ovih polja su označena kao `READONLY` - ona se vraćaju od strane API-ja ali ih nije moguće postaviti.

### Struktura teksta komentara

Komentari se pišu u FastComments varijanti Markdowna, što je jednostavno Markdown plus tradicionalne `bbcode` oznake za slike, poput `[img]path[/img]`.

Tekst je pohranjen u dva polja. Tekst koji je korisnik unio pohranjuje se nepromijenjen u polje `comment`. On se renderira i pohranjuje u polje `commentHTML`.

Dozvoljeni HTML tagovi su `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Preporučuje se renderirati HTML, budući da je u pitanju vrlo mali podskup HTML-a, izrada renderer-a je prilično jednostavna. Postoji više biblioteka za React Native i Flutter, na primjer, koje mogu pomoći u tome

Možete odabrati renderiranje nenormalizirane vrijednosti polja `comment`. [Primjer parsera je ovdje.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primjer parsera se također može prilagoditi za rad s HTML-om i transformirati HTML tagove u očekivane elemente za renderiranje na vašoj platformi. 

### Označavanje

Kada su korisnici označeni u komentaru, informacije se pohranjuju u listu nazvanu `mentions`. Svaki objekt u toj listi ima sljedeću strukturu.

[inline-code-attrs-start title = 'Struktura CommentUserMention objekta'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, ovo će imati prefiks vašeg tenant id-a. **/
    id: string
    /** Konačni tekst @mention oznake, uključujući simbol @. **/
    tag: string
    /** Originalni tekst @mention oznake, uključujući simbol @. **/
    rawTag: string
    /** Koje vrste korisnik je bio označen. user = FastComments.com račun. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od obavijesti, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

### Hashtagovi

Kada se koriste hashtagovi i uspješno se parsiraju, informacije se pohranjuju u listu nazvanu `hashTags`. Svaki objekt u toj listi ima sljedeću strukturu. Hashtagove je moguće također ručno dodati u niz `hashTags` komentara za upite, ako je `retain` postavljeno.

[inline-code-attrs-start title = 'Struktura CommentHashTag objekta'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ID hashtaga. **/
    id: string
    /** Konačni tekst #hashtag oznake, uključujući simbol #. **/
    tag: string
    /** Ako je hashtag povezan s prilagođenim URL-om, ovo će biti definirano. **/
    url?: string
    /** Ako trebamo zadržati hashtag, čak i ako ne postoji u tekstu komentara, kada se komentar ažurira. Korisno za označavanje komentara bez mijenjanja teksta komentara. **/
    retain?: boolean
}
[inline-code-end]