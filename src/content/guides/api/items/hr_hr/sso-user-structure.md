FastComments pruža jednostavno za korištenje SSO rješenje. Ažuriranje informacija o korisniku pomoću integracije temeljenoj na HMAC-u jednostavno je kao da korisnik učita stranicu s ažuriranim payloadom.

Međutim, može biti poželjno upravljati korisnikom izvan tog tijeka kako bi se poboljšala konzistentnost vaše aplikacije.

SSO User API pruža način za CRUD objekata koje nazivamo SSOUsers. Ti su objekti različiti od običnih Users i čuvaju se odvojeno radi sigurnosti tipova.

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
    isAccountOwner?: boolean // Administratorska dozvola - SSO korisnici s ovom zastavicom naplaćuju se kao SSO administratori (odvojeno od običnih SSO korisnika)
    isAdminAdmin?: boolean // Administratorska dozvola - SSO korisnici s ovom zastavicom naplaćuju se kao SSO administratori (odvojeno od običnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Moderatorska dozvola - SSO korisnici s ovom zastavicom naplaćuju se kao SSO moderatori (odvojeno od običnih SSO korisnika)
    /** Ako je null, Kontrola pristupa se neće primijeniti na korisnika. Ako je prazna lista, ovaj korisnik neće moći vidjeti nijedne stranice niti @mention-ati druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dopustite drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njegovom profilu. Zadano je true kako bi profili bili sigurni po defaultu. **/
    isProfileActivityPrivate?: boolean
    /** Ne dopustite drugim korisnicima da ostavljaju komentare na korisnikovom profilu ili da vide postojeće komentare na profilu. Zadano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dopustite drugim korisnicima da šalju izravne poruke ovom korisniku. Zadano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcionalna konfiguracija za korisničke značke. **/
    badgeConfig?: {
        /** Niz ID-eva znački koje će se dodijeliti korisniku. Ograničeno na 30 znački. Redoslijed se poštuje. To su globalne značke vidljive na svim stranicama. **/
        badgeIds: string[]
        /** Niz ID-eva znački ograničenih na trenutnu stranicu (urlId). Ove značke se prikazuju samo na stranici na kojoj su dodijeljene. **/
        pageBadgeIds?: string[]
        /** Ako je true, zamjenjuje sve postojeće prikazane značke s navedenima. Globalne i na stranici ograničene značke pregazi se neovisno. Ako je false, dodaje se postojećim značkama. **/
        override?: boolean
        /** Ako je true, ažurira svojstva prikaza znački iz konfiguracije tenanta. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju drugačije ovisno o njihovim zastavicama dozvola:

- **Regular SSO Users**: Korisnici bez administratorskih ili moderatorskih dozvola naplaćuju se kao obični SSO korisnici
- **SSO Admins**: Korisnici s oznakama `isAccountOwner` ili `isAdminAdmin` naplaćuju se zasebno kao SSO Admini (isti tarifni razred kao regularni administratori tenanta)
- **SSO Moderators**: Korisnici s oznakom `isCommentModeratorAdmin` naplaćuju se zasebno kao SSO Moderatori (isti tarifni razred kao regularni moderatori)

**Važno**: Kako bi se spriječilo dvostruko naplaćivanje, sustav automatski uklanja duplikate SSO korisnika u odnosu na regularne tenant korisnike i moderatore prema adresi e-pošte. Ako SSO korisnik ima istu email adresu kao regularni tenant korisnik ili moderator, neće biti naplaćen dvaput.

### Kontrola pristupa

Korisnici se mogu podijeliti u grupe. Za to služi polje `groupIds`, i ono je opcionalno.

### @Mentions

Po zadanim postavkama `@mentions` koristi `username` za pretraživanje drugih sso korisnika kada se upiše znak `@`. Ako se koristi `displayName`, tada će se rezultati koji odgovaraju `username` zanemariti kada postoji podudaranje za `displayName`, i rezultati pretraživanja `@mention` će koristiti `displayName`.

### Pretplate

Uz FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widgetu za komentare i odabirom Pretplati se.

Kod regularnog korisnika, šaljemo im obavijesti e-poštom na temelju njihovih postavki obavijesti.

Kod SSO korisnika to smo razdvojili radi kompatibilnosti unatrag. Korisnici će dobivati ove dodatne e-poruke za obavijesti o pretplati tek ako postavite `optedInSubscriptionNotifications` na `true`.

### Značke

Možete dodijeliti značke SSO korisnicima koristeći svojstvo `badgeConfig`. Značke su vizualni indikatori koji se pojavljuju uz ime korisnika u komentarima.

- `badgeIds` - Niz ID-eva znački koje će se dodijeliti korisniku. To su globalne značke vidljive na svim stranicama. Moraju biti valjani ID-evi znački kreirani u vašem FastComments računu. Ograničeno na 30 znački.
- `pageBadgeIds` - Opcionalni niz ID-eva znački ograničenih na trenutnu stranicu (`urlId`). Ove značke se prikazuju samo na stranici na kojoj su dodijeljene. Različite stranice mogu imati različite značke ograničene na stranicu za istog korisnika.
- `override` - Ako je true, sve postojeće prikazane značke bit će zamijenjene navedenima. Globalne i značke ograničene na stranicu zamjenjuju se neovisno — zamjena globalnih znački ne utječe na značke ograničene na stranicu i obratno. Ako je false ili izostavljeno, navedene značke će se dodati postojećim značkama.
- `update` - Ako je true, svojstva prikaza znački će se ažurirati iz konfiguracije tenanta kad god se korisnik prijavi.