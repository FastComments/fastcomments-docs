Objekt `Comment` predstavlja komentar koji je ostavio korisnik.

Odnos između roditeljskih i podređenih komentara definira se putem `parentId`.

Struktura objekta Comment je sljedeća:

[inline-code-attrs-start title = 'Struktura komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: Postavljeno na true ako je antispam mehanizam utvrdio da je komentar spam. **/
    aiDeterminedSpam?: boolean
    /** Je li komentar odobren za prikaz. Postaviti na true prilikom spremanja komentara, inače će biti skriven. **/
    approved?: boolean
    /** Avatar korisnika. **/
    avatarSrc?: string
    /** Podređeni komentari. Nisu popunjeni u svim scenarijima. Koriste se kada je asTree postavljeno na true putem API-ja. **/
    children: Comment[]
    /** Izvorni komentar autora. **/
    comment: string
    /** READONLY: Komentar autora parsiran u HTML. **/
    commentHTML?: string
    /** Email autora komentara. Obavezno ako je anonimno komentiranje isključeno. **/
    commenterEmail?: string
    /** Link autora komentara (na primjer, njihov blog). **/
    commenterLink?: string
    /** Ime autora komentara. Uvijek obavezno. Ako nije dostupno, postavite nešto poput "Anoniman". **/
    commenterName: string
    /** Datum kada je komentar ostavljen, u UTC epoch formatu. **/
    date: number
    /** "Prikazna oznaka" komentara - na primjer "Admin", "Moderator", ili nešto poput "VIP User". **/
    displayLabel?: string
    /** Domena na kojoj je komentar objavljen. **/
    domain?: string
    /** READONLY: Broj puta koliko je komentar prijavljen. **/
    flagCount?: number
    /** #hashtagovi napisani u komentaru koji su uspješno parsirani. Također možete ručno dodati hashtagove za pretraživanje, ali oni se neće automatski prikazati u tekstu komentara. **/
    hashTags?: CommentHashTag[]
    /** READONLY: Sadrži li komentar slike? **/
    hasImages?: boolean
    /** READONLY: Sadrži li komentar linkove? **/
    hasLinks?: boolean
    /** READONLY: Jedinstveni ID komentara. **/
    id: string
    /** Samo pri kreiranju! Ovo se hashira za pohranu. **/
    ip?: string
    /** READONLY: Je li trenutni korisnik blokirao korisnika koji je napisao ovaj komentar? **/
    isBlocked?: boolean
    /** READONLY: Je li komentar od administratora? Automatski postavljeno na temelju userId. **/
    isByAdmin?: boolean
    /** READONLY: Je li komentar od moderatora? Automatski postavljeno na temelju userId. **/
    isByModerator?: boolean
    /** Postaviti na true ako je komentar privremeno obrisan (morao je biti ostavljen zamjenski zapis zbog neke druge konfiguracije). **/
    isDeleted?: boolean
    /** Postaviti na true ako je račun korisnika obrisan, a komentar je morao biti sačuvan. **/
    isDeletedUser?: boolean
    /** READONLY: Je li komentar označen od trenutno prijavljenog korisnika (contextUserId)? **/
    isFlagged?: boolean
    /** Je li komentar prikvačen? **/
    isPinned?: boolean
    /** Je li komentar zaključan za nove odgovore (moderatori još uvijek mogu odgovarati)? **/
    isLocked?: boolean
    /** Je li komentar spam? **/
    isSpam?: boolean
    /** READONLY: Je li komentar ocijenjen negativno od trenutnog korisnika (contextUserId)? **/
    isVotedDown?: boolean
    /** READONLY: Je li komentar ocijenjen pozitivno od trenutnog korisnika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokalizacija u kojoj je komentar. Ako nije navedeno, izvest će se iz Accept-Language HTTP zaglavlja. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: @mentions napisani u komentaru koji su uspješno parsirani. **/
    mentions?: CommentUserMention[]
    /** Opcionalni meta-podaci povezani s komentarom. **/
    meta?: Record<string, string | number | boolean>
    /** Opcionalna lista ID-eva grupa za moderaciju povezanih s ovim komentarom. **/
    moderationGroupIds?: string[]|null
    /** READONLY: ID objekta glasovanja koji odgovara glasu trenutnog korisnika (contextUserId) na ovom komentaru. **/
    myVoteId?: string
    /** Jesu li za ovaj komentar poslane obavijesti komentatorima. Kako biste spriječili slanje obavijesti prilikom uvoza, postavite ovo na true. **/
    notificationSentForParent?: boolean
    /** Jesu li za ovaj komentar poslane obavijesti korisnicima tenanta. Kako biste spriječili slanje obavijesti prilikom uvoza, postavite ovo na true. **/
    notificationSentForParentTenant?: boolean
    /** Naslov stranice na kojoj je ovaj komentar bio. **/
    pageTitle?: string
    /** Ako odgovaramo na komentar, ovo je ID na koji odgovaramo. **/
    parentId?: string|null
    /** Je li komentar označen kao pregledan. **/
    reviewed: boolean
    /** ID tenanta kojem komentar pripada. **/
    tenantId: string
    /** Korisnik koji je napisao komentar. Automatski se stvara prilikom spremanja komentara s imenom/emailom. **/
    userId?: string|null
    /** URL lokacije na kojoj je ovaj komentar vidljiv, poput objave na blogu. **/
    url: string
    /** "Očišćena" verzija urlId-a koji ste nam poslali. Prilikom spremanja specificirate ovo polje, ali kada dohvatite komentar nazad, ovo će biti "očišćeno" i vaša originalna vrijednost premještena u "urlIdRaw". **/
    urlId: string
    /** READONLY: Izvorni urlId koji ste nam poslali. **/
    urlIdRaw?: string
    /** Je li korisnik i ovaj komentar verificiran? **/
    verified: boolean
    /** Broj pozitivnih glasova. **/
    votesUp?: number
    /** Broj negativnih glasova. **/
    votesDown?: number
    /** "Karma" komentara (= pozitivni glasovi - negativni glasovi). **/
    votes?: number
}
[inline-code-end]

Neka od ovih polja su označena kao `READONLY` - ona se vraćaju iz API-ja, ali se ne mogu postaviti.

### Struktura teksta komentara

Komentari su pisani u FastComments varijanti Markdowna, što je običan Markdown uz tradicionalne `bbcode` stil oznake za slike, poput `[img]path[/img]`.

Tekst se pohranjuje u dva polja. Tekst koji je korisnik unio pohranjuje se neizmijenjen u polje `comment`. Taj tekst se renderira i pohranjuje u polje `commentHTML`.

Dozvoljene HTML oznake su `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Preporučuje se renderiranje HTML-a; budući da je riječ o vrlo malom podskupu HTML-a, izrada renderera je relativno jednostavna. Na primjer, postoje više knjižnica za React Native i Flutter koje mogu pomoći s tim

Možete odabrati renderiranje ne-normiranog sadržaja polja `comment`. [Primjer parsera nalazi se ovdje.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primjer parsera također se može prilagoditi za rad s HTML-om te transformirati HTML oznake u očekivane elemente za renderiranje na vašoj platformi. 

### Označavanje

Kada su korisnici označeni u komentaru, informacija se pohranjuje u listu nazvanu `mentions`. Svaki objekt u toj listi ima sljedeću strukturu.

[inline-code-attrs-start title = 'Objekt spominjanja komentara'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, imat će prefiks vašeg tenant ID-a. **/
    id: string
    /** Konačni tekst @mention taga, uključujući simbol @. **/
    tag: string
    /** Izvorni tekst @mention taga, uključujući simbol @. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com račun. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od obavijesti, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

### HashTags

Kada se koriste hashtagovi i uspješno su parsirani, informacija se pohranjuje u listu nazvanu `hashTags`. Svaki objekt u toj listi ima sljedeću strukturu. Hashtagovi se također mogu ručno dodati u niz `hashTags` komentara za upite, ako je `retain` postavljen.

[inline-code-attrs-start title = 'Objekt hashtagova komentara'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ID hashtaga. **/
    id: string
    /** Konačni tekst #hashtag taga, uključujući simbol #. **/
    tag: string
    /** Ako je hashtag povezan s prilagođenim URL-om, ovo će biti definirano. **/
    url?: string
    /** Ako trebamo zadržati hashtag, čak i ako ne postoji u tekstu komentara, kada se komentar ažurira. Korisno za označavanje komentara bez promjene teksta komentara. **/
    retain?: boolean
}
[inline-code-end]

---