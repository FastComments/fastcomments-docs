FastComments pruža lako rešenje za SSO. Ažuriranje informacija o korisniku pomoću HMAC-integracije
je jednostavno kao da korisnik učita stranicu sa ažuriranim payload-om.

Međutim, može biti poželjno upravljati korisnikom izvan tog toka, radi poboljšanja konzistentnosti vaše aplikacije.

SSO User API pruža način za CRUD objekata koje nazivamo SSOUsers. Ovi objekti se razlikuju od regularnih Users i
držani su odvojeno radi tip sigurnosti.

The structure for the SSOUser object is as follows:

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
    isAccountOwner?: boolean // Dozvola administratora - SSO users sa ovom oznakom se naplaćuju kao SSO Admins (odvojeno od regularnih SSO users)
    isAdminAdmin?: boolean // Dozvola administratora - SSO users sa ovom oznakom se naplaćuju kao SSO Admins (odvojeno od regularnih SSO users)
    isCommentModeratorAdmin?: boolean // Dozvola moderatora - SSO users sa ovom oznakom se naplaćuju kao SSO Moderators (odvojeno od regularnih moderators)
    /** Ako je null, kontrola pristupa se neće primenjivati na korisnika. Ako je prazna lista, ovaj korisnik neće moći da vidi nijednu stranicu ili da @mention-uje druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dozvoliti drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njihovom profilu. Podrazumevano je true kako bi profili bili bezbedni po default-u. **/
    isProfileActivityPrivate?: boolean
    /** Ne dozvoliti drugim korisnicima da ostavljaju komentare na profilu korisnika, niti da vide postojeće komentare na profilu. Podrazumevano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvoliti drugim korisnicima da šalju direktne poruke ovom korisniku. Podrazumevano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcioni konfiguracioni parametar za bedževe korisnika. **/
    badgeConfig?: {
        /** Niz ID-jeva bedževa koji će biti dodeljeni korisniku. Ograničeno na 30 bedževa. Redosled se poštuje. Ovo su globalni bedževi vidljivi na svim stranicama. **/
        badgeIds: string[]
        /** Niz ID-jeva bedževa ograničenih na trenutnu stranicu (urlId). Ovi bedževi se prikazuju samo na stranici na kojoj su dodeljeni. **/
        pageBadgeIds?: string[]
        /** Ako je true, zamenjuje sve postojeće prikazane bedževe sa navedenim. Globalni i stranački bedževi se nadpisuju nezavisno. Ako je false, dodaje se postojećim bedževima. **/
        override?: boolean
        /** Ako je true, ažurira svojstva prikaza bedževa iz tenant konfiguracije. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju drugačije na osnovu njihovih privilegija:

- **Regularni SSO korisnici**: Korisnici bez administratorskih ili moderatorskih privilegija se naplaćuju kao regularni SSO korisnici
- **SSO administratori**: Korisnici sa `isAccountOwner` ili `isAdminAdmin` zastavicama se naplaćuju odvojeno kao SSO Admins (isto kao regularni tenant administratori)
- **SSO moderatorii**: Korisnici sa `isCommentModeratorAdmin` zastavicom se naplaćuju odvojeno kao SSO Moderators (isto kao regularni moderators)

**Važno**: Da bi se sprečilo dvostruko naplaćivanje, sistem automatski uklanja duplikate SSO korisnika u odnosu na regularne tenant korisnike i moderatore po email adresi. Ako SSO korisnik ima isti email kao regularni tenant korisnik ili moderator, neće biti naplaćen dva puta.

### Kontrola pristupa

Korisnici mogu biti podeljeni u grupe. Za to služi polje `groupIds`, i ono je opciono.

### @Mentions

Po default-u `@mentions` koristiće `username` za pretragu drugih sso korisnika kada se ukuca karakter `@`. Ako se koristi `displayName`, rezultati koji odgovaraju
`username` biće ignorisani kada postoji poklapanje za `displayName`, i rezultati pretrage za `@mention` će koristiti `displayName`.

### Pretplate

Sa FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widget-u za komentare i izborom Subscribe.

Kod regularnog korisnika, šaljemo im email obaveštenja na osnovu njihovih podešavanja obaveštenja.

Kod SSO korisnika, razdvajamo ovo radi kompatibilnosti unazad. Korisnici će dobijati ova dodatna obaveštenja o pretplati
samo ako podesite `optedInSubscriptionNotifications` na `true`.

### Bedževi

Možete dodeliti bedževe SSO korisnicima koristeći svojstvo `badgeConfig`. Bedževi su vizuelni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-jeva bedževa koji će biti dodeljeni korisniku. Ovo su globalni bedževi vidljivi na svim stranicama. Moraju biti validni ID-jevi bedževa kreirani u vašem FastComments nalogu. Ograničeno na 30 bedževa.
- `pageBadgeIds` - Opcioni niz ID-jeva bedževa ograničenih na trenutnu stranicu (`urlId`). Ovi bedževi se prikazuju samo na stranici na kojoj su dodeljeni. Različite stranice mogu imati različite stranačke bedževe za istog korisnika.
- `override` - Ako je true, svi postojeći prikazani bedževi će biti zamenjeni navedenim. Globalni i stranački bedževi se nadpisuju nezavisno — nadpisivanje globalnih bedževa ne utiče na stranačke bedževe i obrnuto. Ako je false ili izostavljeno, navedeni bedževi će biti dodati postojećim.
- `update` - Ako je true, svojstva prikaza bedževa će biti ažurirana iz tenant konfiguracije kad god se korisnik prijavi.