Objekat `Comment` predstavlja komentar koji je ostavio korisnik.

Odnos između roditeljskih i podređenih komentara definiše se preko `parentId`.

Struktura objekta Comment je sledeća:

[inline-code-attrs-start title = 'Struktura komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** SAMO ZA ČITANJE: Postavljeno na true ako je spam mehanizam utvrdio da je komentar spam. **/
    aiDeterminedSpam?: boolean
    /** Da li je komentar odobren za prikaz. Postavi se na true prilikom čuvanja komentara, inače će biti sakriven. **/
    approved?: boolean
    /** Avatar korisnika. **/
    avatarSrc?: string
    /** Podkomentari. Nisu popunjeni u svim scenarijima. Koriste se kada je asTree podešeno na true preko API-ja. **/
    children: Comment[]
    /** Originalni komentar autora. **/
    comment: string
    /** SAMO ZA ČITANJE: Komentar autora parsiran u HTML. **/
    commentHTML?: string
    /** Email autora komentara. Obavezno ako je anonimno komentarisanje isključeno. **/
    commenterEmail?: string
    /** Link autora komentara (na primer, njihov blog). **/
    commenterLink?: string
    /** Ime autora komentara. Uvek obavezno. Ako nije dostupno, postavite nešto poput "Anonymous". **/
    commenterName: string
    /** Datum kada je komentar ostavljen, u UTC epoch formatu. **/
    date: number
    /** "Prikazna oznaka" za komentar - na primer "Admin", "Moderator", ili nešto kao "VIP User". **/
    displayLabel?: string
    /** Domen na kojem je komentar objavljen. **/
    domain?: string
    /** SAMO ZA ČITANJE: Broj puta koliko je komentar prijavljen. **/
    flagCount?: number
    /** Hashtagovi (#) napisani u komentaru koji su uspešno parsirani. Takođe možete ručno dodati hashtagove za pretragu, ali oni se neće automatski prikazivati u tekstu komentara. **/
    hashTags?: CommentHashTag[]
    /** SAMO ZA ČITANJE: Da li komentar sadrži slike? **/
    hasImages?: boolean
    /** SAMO ZA ČITANJE: Da li komentar sadrži linkove? **/
    hasLinks?: boolean
    /** SAMO ZA ČITANJE: Jedinstveni id komentara. **/
    id: string
    /** Samo pri kreiranju! Ovo se hešira za čuvanje. **/
    ip?: string
    /** SAMO ZA ČITANJE: Da li je trenutni korisnik blokirao korisnika koji je napisao ovaj komentar? **/
    isBlocked?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar napisao administrator? Automatski se postavlja na osnovu userId. **/
    isByAdmin?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar napisao moderator? Automatski se postavlja na osnovu userId. **/
    isByModerator?: boolean
    /** Postaviti na true ako je komentar "soft deleted" (ostavljen je zamenski zapis zbog neke druge konfiguracije). **/
    isDeleted?: boolean
    /** Postaviti na true ako je nalog korisnika obrisan, a komentar je morao biti zadržan. **/
    isDeletedUser?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar prijavljen od strane trenutno prijavljenog korisnika (contextUserId)? **/
    isFlagged?: boolean
    /** Da li je komentar pinovan? **/
    isPinned?: boolean
    /** Da li je komentar zaključan za nove odgovore (moderatori i dalje mogu odgovoriti)? **/
    isLocked?: boolean
    /** Da li je komentar spam? **/
    isSpam?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar ocenjen negativno od strane trenutnog korisnika (contextUserId)? **/
    isVotedDown?: boolean
    /** SAMO ZA ČITANJE: Da li je komentar ocenjen pozitivno od strane trenutnog korisnika (contextUserId)? **/
    isVotedUp?: boolean
    /** Lokal (jezik) komentara. Ako nije naveden, biće izveden iz Accept-Language HTTP zaglavlja. **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** SAMO ZA ČITANJE: @pominjanja napisana u komentaru koja su uspešno parsirana. **/
    mentions?: CommentUserMention[]
    /** Opcioni meta-podaci povezani sa komentarom. **/
    meta?: Record<string, string | number | boolean>
    /** Opcionalna lista ID-jeva grupa za moderaciju povezanih sa ovim komentarom. **/
    moderationGroupIds?: string[]|null
    /** SAMO ZA ČITANJE: ID objekta glasanja koji odgovara glasu trenutnog korisnika (contextUserId) za ovaj komentar. **/
    myVoteId?: string
    /** Da li su notifikacije poslate za ovaj komentar komentatorima. Da biste sprečili slanje notifikacija prilikom uvoza, postavite ovo na true. **/
    notificationSentForParent?: boolean
    /** Da li su notifikacije poslate za ovaj komentar korisnicima tenanta. Da biste sprečili slanje notifikacija prilikom uvoza, postavite ovo na true. **/
    notificationSentForParentTenant?: boolean
    /** Naslov stranice na kojoj se ovaj komentar nalazi. **/
    pageTitle?: string
    /** Ako odgovaramo na komentar, ovo je ID na koji odgovaramo. **/
    parentId?: string|null
    /** Da li je komentar označen kao pregledan. **/
    reviewed: boolean
    /** ID tenanta kome komentar pripada. **/
    tenantId: string
    /** Korisnik koji je napisao komentar. Kreira se automatski prilikom čuvanja komentara sa imenom/email-om. **/
    userId?: string|null
    /** URL lokacije na kojoj je ovaj komentar vidljiv, na primer objava na blogu. **/
    url: string
    /** "Očišćena" verzija urlId koju ste prosledili. Prilikom čuvanja specificirate ovo polje, ali kada dobijete komentar nazad ovo će biti "očišćeno" i vaša originalna vrednost prebačena u "urlIdRaw". **/
    urlId: string
    /** SAMO ZA ČITANJE: Originalni urlId koji ste prosledili. **/
    urlIdRaw?: string
    /** Da li su korisnik i ovaj komentar verifikovani? **/
    verified: boolean
    /** Broj pozitivnih glasova. **/
    votesUp?: number
    /** Broj negativnih glasova. **/
    votesDown?: number
    /** "Karma" komentara (= broj pozitivnih glasova - broj negativnih glasova). **/
    votes?: number
}
[inline-code-end]

Neka od ovih polja su označena kao `READONLY` - ta polja se vraćaju iz API-ja, ali se ne mogu postaviti.

### Struktura teksta komentara

Komentari su napisani u FastComments varijanti markdown-a, koja je samo markdown plus tradicionalne `bbcode` stil oznake za slike, kao što je `[img]path[/img]`.

Tekst se čuva u dva polja. Tekst koji je korisnik uneo se čuva neizmenjen u polju `comment`. Ovo se renderuje i čuva u polju `commentHTML`.

Dozvoljeni HTML tagovi su `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

Preporučuje se renderovanje HTML-a, s obzirom da je to vrlo mali podskup HTML-a, pa je izgradnja renderera prilično jednostavna. Na primer, postoji više biblioteka za React Native i Flutter koje mogu pomoći u tome

Možete izabrati da renderujete ne-normalizovanu vrednost polja `comment`. [Primer parsera je ovde.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

Primer parsera se takođe može prilagoditi da radi sa HTML-om i da transformiše HTML tagove u očekivane elemente za renderovanje na vašoj platformi. 

### Obeležavanje

Kada su korisnici označeni u komentaru, informacije se čuvaju u listi nazvanoj `mentions`. Svaki objekat u toj listi
ima sledeću strukturu.

[inline-code-attrs-start title = 'Objekat pomena komentara'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ID korisnika. Za SSO korisnike, biće prefiksiran vašim tenant ID-jem. **/
    id: string
    /** Konačni tekst @mention tag-a, uključujući simbol @. **/
    tag: string
    /** Originalni tekst @mention tag-a, uključujući simbol @. **/
    rawTag: string
    /** Koji tip korisnika je označen. user = FastComments.com nalog. sso = SSOUser. **/
    type: 'user'|'sso'
    /** Ako se korisnik odjavi od notifikacija, ovo će i dalje biti postavljeno na true. **/
    sent: boolean
}
[inline-code-end]

### Hashtagovi

Kada se hashtagovi koriste i uspešno parsiraju, informacije se čuvaju u listi nazvanoj `hashTags`. Svaki objekat u toj listi
ima sledeću strukturu. Hashtagove takođe možete ručno dodati u niz `hashTags` komentara za pretragu, ako je `retain` postavljeno.

[inline-code-attrs-start title = 'Objekat Hashtag-a komentara'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ID hashtaga. **/
    id: string
    /** Konačni tekst #hashtag taga, uključujući simbol #. **/
    tag: string
    /** Ako je hashtag povezan sa prilagođenim URL-om, ovo će biti definisano. **/
    url?: string
    /** Da li treba zadržati hashtag, čak i ako ne postoji u tekstu komentara, kada se komentar ažurira. Korisno za označavanje komentara bez menjanja teksta komentara. **/
    retain?: boolean
}
[inline-code-end]

---