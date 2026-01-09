FastComments pruža jednostavno SSO rješenje. Ažuriranje korisničkih podataka s HMAC-integracijom je jednostavno kao da korisnik učita stranicu s ažuriranim payloadom.

Međutim, može biti poželjno upravljati korisnikom izvan tog tijeka, kako biste poboljšali konzistentnost vaše aplikacije.

SSO User API pruža način za CRUD objekata koje nazivamo SSOUsers. Ti su objekti različiti od običnih korisnika i držani su odvojeno radi sigurnosti tipova.

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
    isAccountOwner?: boolean // Dozvola administratora - SSO korisnici s ovom oznakom naplaćuju se kao SSO administratori (odvojeno od običnih SSO korisnika)
    isAdminAdmin?: boolean // Dozvola administratora - SSO korisnici s ovom oznakom naplaćuju se kao SSO administratori (odvojeno od običnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Dozvola moderatora - SSO korisnici s ovom oznakom naplaćuju se kao SSO moderatori (odvojeno od običnih SSO korisnika)
    /** Ako je null, Kontrola pristupa neće biti primijenjena na korisnika. Ako je prazna lista, ovaj korisnik neće moći vidjeti nijednu stranicu niti spominjati druge korisnike pomoću @. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dopustite drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njegovom profilu. Zadana vrijednost je true kako bi profili bili sigurni prema zadanim postavkama. **/
    isProfileActivityPrivate?: boolean
    /** Ne dopuštajte drugim korisnicima da ostavljaju komentare na korisnikov profil niti da vide postojeće komentare na profilu. Zadana vrijednost je false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da šalju izravne poruke ovom korisniku. Zadana vrijednost je false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcionalna konfiguracija znački za korisnika. **/
    badgeConfig?: {
        /** Niz ID-eva znački koji će biti dodijeljeni korisniku. Ograničeno na 30 znački. Redoslijed se poštuje. **/
        badgeIds: string[]
        /** Ako je true, zamjenjuje sve postojeće prikazane značke s navedenima. Ako je false, dodaje se postojećim značkama. **/
        override?: boolean
        /** Ako je true, ažurira svojstva prikaza znački iz konfiguracije tenanta. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju različito ovisno o njihovim oznakama dozvola:

- **Regular SSO Users**: Korisnici bez administratorskih ili moderatorskih dozvola naplaćuju se kao regularni SSO korisnici
- **SSO Admins**: Korisnici s oznakama `isAccountOwner` ili `isAdminAdmin` naplaćuju se odvojeno kao SSO administratori (ista stopa kao obični administratori tenanta)
- **SSO Moderators**: Korisnici s oznakom `isCommentModeratorAdmin` naplaćuju se odvojeno kao SSO moderatori (ista stopa kao obični moderatori)

**Važno**: Kako bi se spriječilo dvostruko naplaćivanje, sustav automatski deduplikira SSO korisnike naspram običnih tenant korisnika i moderatora po e-mail adresi. Ako SSO korisnik ima istu e-mail adresu kao običan tenant korisnik ili moderator, neće biti naplaćeni dvaput.

### Kontrola pristupa

Korisnici se mogu podijeliti u grupe. To je svrha polja `groupIds`, i ono je opcionalno.

### @Mentions

Po defaultu `@mentions` će koristiti `username` za pretraživanje drugih SSO korisnika kada se upiše znak `@`. Ako se koristi `displayName`, rezultati koji odgovaraju
`username` će biti zanemareni kada postoji podudaranje za `displayName`, te će rezultati pretraživanja `@mention` koristiti `displayName`.

### Pretplate

S FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widgetu komentara i pritiskom na Pretplati se.

Kod običnog korisnika, šaljemo im obavijesti putem e-pošte na temelju njihovih postavki obavijesti.

Kod SSO korisnika, to razdvajamo radi kompatibilnosti s prethodnim verzijama. Korisnici će dobivati ove dodatne e-poruke o pretplati
samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Značke

Možete dodijeliti značke SSO korisnicima koristeći svojstvo `badgeConfig`. Značke su vizualni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-eva znački koje treba dodijeliti korisniku. Moraju biti valjani ID-evi znački kreirani na vašem FastComments računu. Ograničeno na 30 znački.
- `override` - Ako je true, sve postojeće značke prikazane u komentarima bit će zamijenjene navedenima. Ako je false ili izostavljeno, navedene značke će se dodati postojećim značkama.
- `update` - Ako je true, svojstva prikaza znački bit će ažurirana iz konfiguracije tenanta svaki put kada se korisnik prijavi.

---