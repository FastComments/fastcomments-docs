FastComments pruža jednostavno SSO rešenje. Ažuriranje informacija o korisniku pomoću HMAC-integracije je
jednostavno kao učitavanje stranice od strane korisnika sa ažuriranim payload-om.

Međutim, može biti poželjno upravljati korisnikom van tog toka, radi poboljšanja konzistentnosti vaše aplikacije.

SSO User API pruža način za CRUD objekata koje nazivamo SSOUsers. Ovi objekti se razlikuju od regularnih Users i
držani su odvojeno radi bezbednosti tipova.

Struktura SSOUser objekta je sledeća:

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
    isAccountOwner?: boolean // Dozvola administratora - SSO korisnici sa ovom zastavicom se fakturišu kao SSO Admins (odvojeno od regularnih SSO korisnika)
    isAdminAdmin?: boolean // Dozvola administratora - SSO korisnici sa ovom zastavicom se fakturišu kao SSO Admins (odvojeno od regularnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Dozvola moderatora - SSO korisnici sa ovom zastavicom se fakturišu kao SSO Moderators (odvojeno od regularnih SSO korisnika)
    /** Ako je null, Kontrola pristupa neće biti primenjena na korisnika. Ako je prazan spisak, ovaj korisnik neće moći da vidi nijednu stranicu niti da @mention-uje druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dozvolite drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njegovom profilu. Podrazumevano je true kako bi profili bili podrazumevano sigurni. **/
    isProfileActivityPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da ostavljaju komentare na korisnikovom profilu, niti da vide postojeće komentare na profilu. Podrazumevano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da šalju direktne poruke ovom korisniku. Podrazumevano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcionalna konfiguracija za značke korisnika. **/
    badgeConfig?: {
        /** Niz ID-jeva znački koje će biti dodeljene korisniku. Ograničeno na 30 znački. Redosled se poštuje. **/
        badgeIds: string[]
        /** Ako je true, zamenjuje sve postojeće prikazane značke sa dostavljenim. Ako je false, dodaje se postojećim značkama. **/
        override?: boolean
        /** Ako je true, ažurira svojstva prikaza znački iz konfiguracije tenant-a. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju različito u zavisnosti od njihovih dozvola:

- **Regular SSO Users**: Korisnici bez administratorskih ili moderatorskih dozvola se naplaćuju kao regularni SSO korisnici
- **SSO Admins**: Korisnici sa `isAccountOwner` ili `isAdminAdmin` zastavicama se naplaćuju odvojeno kao SSO Admins (isti tarifni razred kao regularni tenant admini)
- **SSO Moderators**: Korisnici sa `isCommentModeratorAdmin` zastavicom se naplaćuju odvojeno kao SSO Moderators (isti tarifni razred kao regularni moderatori)

**Važno**: Da bi se sprečilo dvostruko naplaćivanje, sistem automatski deduplicira SSO korisnike naspram regularnih tenant korisnika i moderatora po email adresi. Ako SSO korisnik ima isti email kao regularni tenant korisnik ili moderator, neće biti naplaćen dva puta.

### Kontrola pristupa

Korisnici mogu biti podeljeni u grupe. Za to služi polje `groupIds`, i nije obavezno.

### @Mentions

Po podrazumevanoj vrednosti, `@mentions` će koristiti `username` za pretragu drugih sso korisnika kada se unese karakter `@`. Ako se koristi `displayName`, tada će rezultati koji odgovaraju `username` biti zanemareni kada postoji poklapanje za `displayName`, i rezultati pretrage za `@mention` će koristiti `displayName`.

### Pretplate

Sa FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonicu zvona u widgetu za komentare i izborom Subscribe.

Kod regularnog korisnika, šaljemo im email notifikacije na osnovu njihovih podešavanja za notifikacije.

Kod SSO korisnika, razdvajamo ovo radi kompatibilnosti unazad. Korisnici će dobijati ove dodatne emailove o pretplati samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Značke

Možete dodeljivati značke SSO korisnicima pomoću svojstva `badgeConfig`. Značke su vizuelni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-jeva znački koje će biti dodeljene korisniku. Moraju biti važeći ID-jevi znački kreirani na vašem FastComments nalogu. Ograničeno na 30 znački.
- `override` - Ako je true, sve postojeće značke prikazane na komentarima biće zamenjene navedenim. Ako je false ili izostavljeno, navedene značke biće dodate postojećim značkama.
- `update` - Ako je true, svojstva prikaza znački će se ažurirati iz konfiguracije tenant-a kad god se korisnik prijavi.