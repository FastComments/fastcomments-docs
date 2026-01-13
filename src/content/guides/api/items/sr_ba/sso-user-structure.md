FastComments pruža jednostavno za korištenje SSO rješenje. Ažuriranje informacija o korisniku s integracijom zasnovanom na HMAC-u je jednostavno kao da korisnik učita stranicu sa ažuriranim payload-om.

Međutim, može biti poželjno upravljati korisnikom izvan tog toka, radi bolje konzistentnosti vaše aplikacije.

SSO User API pruža način za CRUD objekata koje zovemo SSOUsers. Ovi objekti se razlikuju od regularnih Users i čuvaju se odvojeno radi tip-sigurnosti.

Struktura SSOUser objekta je sljedeća:

[inline-code-attrs-start title = 'Struktura SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Admin ovlašćenje - SSO korisnici sa ovom oznakom se naplaćuju kao SSO administratori (odvojeno od regularnih SSO korisnika)
    isAdminAdmin?: boolean // Admin ovlašćenje - SSO korisnici sa ovom oznakom se naplaćuju kao SSO administratori (odvojeno od regularnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Moderator ovlašćenje - SSO korisnici sa ovom oznakom se naplaćuju kao SSO moderatori (odvojeno od regularnih SSO korisnika)
    /** Ako je null, Kontrola pristupa se neće primijeniti na korisnika. Ako je prazan spisak, ovaj korisnik neće moći vidjeti nijednu stranicu niti @mention-ovati druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dozvoljava drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njegovom profilu. Podrazumijevano je true radi sigurnih profila. **/
    isProfileActivityPrivate?: boolean
    /** Ne dozvoljava drugim korisnicima da ostavljaju komentare na korisnikovom profilu, niti da vide postojeće komentare na profilu. Podrazumijevano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvoljava drugim korisnicima da šalju direktne poruke ovom korisniku. Podrazumijevano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcionalna konfiguracija za oznake (badges) korisnika. **/
    badgeConfig?: {
        /** Niz ID-jeva oznaka koje će biti dodijeljene korisniku. Ograničeno na 30 oznaka. Redoslijed se poštuje. **/
        badgeIds: string[]
        /** Ako je true, zamjenjuje sve postojeće prikazane oznake sa navedenim. Ako je false, dodaje se postojećim oznakama. **/
        override?: boolean
        /** Ako je true, ažurira prikazne osobine oznaka iz konfiguracije tenanta. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju različito u zavisnosti od njihovih zastavica ovlašćenja:

- **Regular SSO Users**: Korisnici bez admin ili moderator ovlašćenja se naplaćuju kao regularni SSO korisnici
- **SSO Admins**: Korisnici sa `isAccountOwner` ili `isAdminAdmin` zastavicom se naplaćuju odvojeno kao SSO administratori (ista cijena kao regularni tenant administratori)
- **SSO Moderators**: Korisnici sa `isCommentModeratorAdmin` zastavicom se naplaćuju odvojeno kao SSO moderatori (ista cijena kao regularni moderatori)

**Važno**: Da bi se spriječilo dvostruko naplaćivanje, sistem automatski uklanja duplikate SSO korisnika u odnosu na regularne tenant korisnike i moderatore koristeći email adresu. Ako SSO korisnik ima istu email adresu kao regularni tenant korisnik ili moderator, neće biti naplaćen dvaput.

### Kontrola pristupa

Korisnici se mogu podijeliti u grupe. Za to služi polje `groupIds` i opcionalno je.

### @Mentions

Po zadanoj postavci `@mentions` će koristiti `username` za pretragu drugih sso korisnika kada se utipka znak `@`. Ako se koristi `displayName`, rezultati koji odgovaraju `username` biće ignorisani kada postoji podudaranje za `displayName`, i rezultati pretrage `@mention` će koristiti `displayName`.

### Pretplate

Sa FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widgetu za komentare i klikom na Pretplati se.

Kod regularnog korisnika, šaljemo im email obavještenja na osnovu njihovih postavki obavijesti.

Kod SSO korisnika, razdvojili smo ovo radi kompatibilnosti unazad. Korisnici će dobiti ova dodatna email obavještenja o pretplatama samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Oznake

Možete dodijeliti oznake SSO korisnicima koristeći svojstvo `badgeConfig`. Oznake su vizuelni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-jeva oznaka koje treba dodijeliti korisniku. Moraju biti važeći ID-jevi oznaka kreirani na vašem FastComments nalogu. Ograničeno na 30 oznaka.
- `override` - Ako je true, sve postojeće oznake prikazane u komentarima će biti zamijenjene navedenim. Ako je false ili izostavljeno, navedene oznake će biti dodane postojećim oznakama.
- `update` - Ako je true, prikazne osobine oznaka će biti ažurirane iz konfiguracije tenanta svaki put kada se korisnik prijavi.

---