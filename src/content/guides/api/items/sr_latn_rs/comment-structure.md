A `Comment` object predstavlja komentar ostavljen od strane korisnika.

Odnos između roditeljskih i dečijih komentara je definisan putem `parentId`.

Struktura za Comment objekat je sledeća:

[inline-code-attrs-start title = 'Struktura Comment objekta'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SAMO ZA ČITANJE: Podesiti na true ako je anti-spam motor odredio da je komentar spam. **/
    aiDeterminedSpam?: boolean
    /** Da li je komentar odobren za prikaz. Podesi na true prilikom čuvanja komentara, u suprotnom će biti sakriven. **/
    approved?: boolean
    /** Korisnikov avatar. **/
    avatarSrc?: string
    /** Dečiji komentari. Nisu popunjeni u svim scenarijima. Koristi se kada je asTree podešen na true preko API-ja. **/
    children: Comment[]
    /** Sirovi komentar koji je unela osoba. **/
    comment: string
    /** SAMO ZA ČITANJE: Komentar parse-ovan u HTML. **/
    commentHTML?: string
    /** Email osobe koja komentariše. Obavezno ako je anonimno komentarisanje isključeno. **/
    commenterEmail?: string
    /** Link osobe koja komentariše (na primer njihov blog). **/
    commenterLink?: string
    /** Ime osobe koja komentariše. Uvek obavezno. Ako nije dostupno, postavite nešto poput "Anonymous". **/
    commenterName: string
    /** Datum kada je komentar ostavljen, u UTC epoch formatu. **/
    date: number
    /** "Prikazna etiketa" za komentar - na primer "Admin", "Moderator", ili nešto poput "VIP User". **/
    displayLabel?: string
    /** Domen na kojem je komentar objavljen. **/
    domain?: string
    /** SAMO ZA ČITANJE: Broj puta koliko je komentar označen (flagovan). **/
    flagCount?: number
    /** Broj #hashtag-ova upisanih u komentar koji su uspešno parsirani. Takođe možete ručno dodavati heštegove, za pretragu, ali oni se neće automatski prikazivati u tekstu komentara. **/
    hashTags?: CommentHashTag[]
    /** SAMO ZA ČITANJE: Da li komentar sadrži slike? **/
    hasImages?: boolean
    /** SAMO ZA ČITANJE: Da li komentar sadrži linkove? **/
    hasLinks?: boolean
    /** SAMO ZA ČITANJE: Jedinstveni id komentara. **/
    id: string
    /** Samo prilikom kreiranja! Ovo se hešira za skladištenje. **/
    ip?: string
    /** SAMO ZA ČITANJE: Da li je trenutni korisnik blokirao korisnika koji je napisao ovaj komentar? **/
    isBlocked?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar od admina? Automatski se podešava na osnovu userId. **/
    isByAdmin?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar od moderatora? Automatski se podešava na osnovu userId. **/
    isByModerator?: boolean
    /** Podesiti na true ako je komentar privremeno obrisan (morao je ostati rezervni zapis zbog neke druge konfiguracije). **/
    isDeleted?: boolean
    /** Podesiti na true ako je korisnički nalog obrisan, a komentar je morao biti sačuvan. **/
    isDeletedUser?: boolean
    /** SAMO ZA ČITANJE: Da li je komentara označen od strane trenutno prijavljenog korisnika (contextUserId)? **/
    isFlagged?: boolean
    /** Da li je komentar prikvačen (pinned)? **/
    isPinned?: boolean
    /** Da li je komentar zaključan? Kada je true, niko (uključujući moderatore) ne može odgovoriti, izmeniti ili obrisati komentar dok se ne otključa. **/
    isLocked?: boolean
    /** Da li je komentar spam? **/
    isSpam?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar ocenjen negativno od strane trenutnog korisnika (contextUserId)? **/
    isVotedDown?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar ocenjen pozitivno od strane trenutnog korisnika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokal u kojem je komentar. Ako nije naveden, biće izveden iz HTTP Accept-Language zaglavlja. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SAMO ZA ČITANJE: @pomeni upisani u komentar koji su uspešno parsirani. **/
    mentions?: CommentUserMention[]
    /** Opcionalni metapodaci povezani sa komentarom. **/
    meta?: Record<string, string | number | boolean>
    /** Opcionalna lista ID-eva grupa za moderaciju povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
    /** SAMO ZA ČITANJE: ID objekta glasanja koji odgovara glasu trenutnog korisnika (contextUserId) za ovaj komentar. **/
    myVoteId?: string
    /** Da li su za ovaj komentar poslata obaveštenja komentarima. Da biste sprečili slanje obaveštenja prilikom uvoza, postavite ovo na true. **/
    notificationSentForParent?: boolean
    /** Da li su za ovaj komentar poslata obaveštenja tenant korisnicima. Da biste sprečili slanje obaveštenja prilikom uvoza, postavite ovo na true. **/
    notificationSentForParentTenant?: boolean
    /** Naslov stranice na kojoj je ovaj komentar. **/
    pageTitle?: string
    /** Ako odgovaramo na komentar, ovo je ID na koji odgovaramo. **/
    parentId?: string|null
    /** Da li je komentar označen kao pregledan. **/
    reviewed: boolean
    /** ID tenanta kojem komentar pripada. **/
    tenantId: string
    /** Korisnik koji je napisao komentar. Kreira se automatski pri čuvanju komentara sa imenom/email-om. **/
    userId?: string|null
    /** URL lokacije na kojoj je ovaj komentar vidljiv, kao na primer blog post. **/
    url: string
    /** "Očišćena" verzija urlId koju ste poslali. Prilikom čuvanja specificirate ovo polje, ali kada povučete komentar nazad ovo će biti "očišćeno" i vaša originalna vrednost biće pomerena u "urlIdRaw". **/
    urlId: string
    /** SAMO ZA ČITANJE: Originalni urlId koji ste poslali. **/
    urlIdRaw?: string
    /** Da li je korisnik i ovaj komentar verifikovan? **/
    verified: boolean
    /** Broj pozitivnih glasova. **/
    votesUp?: number
    /** Broj negativnih glasova. **/
    votesDown?: number
    /** "Karma" komentara (= votes up - votes down). **/
    votes?: number
}
[inline-code-end]

Neka od ovih polja su označena kao `READONLY` - ona se vraćaju od strane API-ja, ali ne mogu se postaviti.

### Struktura teksta komentara

Komentari se pišu u FastComments varijanti markdown-a, što je standardni markdown plus tradicionalne `bbcode` stil oznake za slike, poput `[img]path[/img]`.

Tekst se čuva u dva polja. Tekst koji je korisnik uneo se čuva neizmenjen u polju `comment`. Ovo se renderuje i čuva u polju `commentHTML`.

Dozvoljeni HTML tagovi su `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Preporučuje se da renderujete HTML, pošto je reč o vrlo malom podskupu HTML-a i izrada renderer-a je prilično jednostavna. Postoji više biblioteka za React Native i Flutter, na primer, koje mogu pomoći u tome.

Možete odlučiti da renderujete ne-normalizovanu vrednost polja `comment`. [Primer parsera je ovde.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primer parsera se takođe može prilagoditi da radi sa HTML-om i transformiše HTML tagove u očekivane elemente za prikaz na vašoj platformi. 

### Označavanje

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi nazvanoj `mentions`. Svaki objekat u toj listi ima sledeću strukturu.

[inline-code-attrs-start title = 'Objekat pomena u komentaru'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, ovo će imati prefiks vašeg tenant ID-a. **/
    id: string
    /** Konačni @mention tag tekst, uključujući @ simbol. **/
    tag: string
    /** Originalni @mention tag tekst, uključujući @ simbol. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

### Haštagovi

Kada se koriste haštagovi i uspešno se parsiraju, informacije se čuvaju u listi nazvanoj `hashTags`. Svaki objekat u toj listi ima sledeću strukturu. Haštagovi se takođe mogu ručno dodati u niz `hashTags` komentara za potrebe pretrage, ako je `retain` podešen.

[inline-code-attrs-start title = 'Objekat haštagova komentara'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ID haštag-a. **/
    id: string
    /** Konačni #hashtag tag tekst, uključujući # simbol. **/
    tag: string
    /** Ako je haštag povezan sa prilagođenim URL-om, ovo će biti definisano. **/
    url?: string
    /** Da li treba da zadržimo haštag, čak i ako se ne nalazi u tekstu komentara, kada se komentar ažurira. Korisno za označavanje komentara bez promene teksta komentara. **/
    retain?: boolean
}
[inline-code-end]

---