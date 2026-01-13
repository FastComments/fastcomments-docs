FastComments pruža lako za korišćenje SSO rešenje. Ažuriranje informacija o korisniku pomoću HMAC-bazirane integracije
je jednostavno kao učitavanje stranice od strane korisnika sa ažuriranim payload-om.

Međutim, može biti poželjno upravljati korisnikom izvan tog toka, kako bi se poboljšala konzistentnost vaše aplikacije.

SSO User API pruža način za CRUD objekata koje nazivamo SSOUsers. Ovi objekti se razlikuju od regularnih Users i
držani su odvojeno radi tip-sigurnosti.

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
    isAccountOwner?: boolean // Administratorska dozvola - SSO korisnici sa ovom oznakom se naplaćuju kao SSO administratori (odvojeno od regularnih SSO korisnika)
    isAdminAdmin?: boolean // Administratorska dozvola - SSO korisnici sa ovom oznakom se naplaćuju kao SSO administratori (odvojeno od regularnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Moderator dozvola - SSO korisnici sa ovom oznakom se naplaćuju kao SSO moderatori (odvojeno od regularnih SSO korisnika)
    /** Ako je null, Kontrola pristupa se neće primjenjivati na korisnika. Ako je prazan spisak, ovaj korisnik neće moći vidjeti nijednu stranicu niti @pomenuti druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dozvoljava drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na profilu. Podrazumevana vrednost je true da bi profili bili sigurni. **/
    isProfileActivityPrivate?: boolean
    /** Ne dozvoljava drugim korisnicima da ostavljaju komentare na korisnikovom profilu, niti da vide postojeće komentare na profilu. Podrazumevano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvoljava drugim korisnicima da šalju direktne poruke ovom korisniku. Podrazumevano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcionalna konfiguracija za bedževe korisnika. **/
    badgeConfig?: {
        /** Niz ID-jeva bedževa koji se dodeljuju korisniku. Ograničeno na 30 bedževa. Redosled se poštuje. **/
        badgeIds: string[]
        /** Ako je true, zamenjuje sve postojeće prikazane bedževe sa ovim. Ako je false, dodaje ih postojećim bedževima. **/
        override?: boolean
        /** Ako je true, ažurira osobine prikaza bedževa iz konfiguracije tenanta. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju drugačije na osnovu njihovih dozvola:

- **Regular SSO Users**: Korisnici bez administrativnih ili moderatorskih dozvola se naplaćuju kao regularni SSO korisnici
- **SSO Admins**: Korisnici sa `isAccountOwner` ili `isAdminAdmin` oznakama se naplaćuju odvojeno kao SSO administratori (ista tarifa kao regularni administratori tenanta)
- **SSO Moderators**: Korisnici sa `isCommentModeratorAdmin` oznakom se naplaćuju odvojeno kao SSO moderatori (ista tarifa kao regularni moderatori)

**Važno**: Da bi se izbegla dupla naplata, sistem automatski deduplikuje SSO korisnike naspram regularnih korisnika tenanta i moderatora po email adresi. Ako SSO korisnik ima isti email kao regularni korisnik tenanta ili moderator, neće biti naplaćen dvaput.

### Kontrola pristupa

Korisnici mogu biti razvrstani u grupe. Za to služi polje `groupIds`, i opcionalno je.

### @Mentions

Po defaultu `@mentions` će koristiti `username` za pretragu drugih sso korisnika kada se unese karakter `@`. Ako se koristi `displayName`, tada će rezultati koji odgovaraju
`username` biti ignorisani kada postoji podudaranje za `displayName`, i rezultati pretrage za `@mention` će koristiti `displayName`.

### Pretplate

Sa FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widgetu za komentare i klikom na Subscribe.

Kod regularnog korisnika, šaljemo im email obaveštenja na osnovu njihovih podešavanja obaveštenja.

Kod SSO korisnika, razdvajamo ovo radi kompatibilnosti unazad. Korisnici će dobijati ova dodatna email obaveštenja o pretplati
samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Bedževi

Možete dodeljivati bedževe SSO korisnicima koristeći svojstvo `badgeConfig`. Bedževi su vizuelni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-jeva bedževa koji se dodeljuju korisniku. Oni moraju biti validni ID-jevi bedževa kreirani na vašem FastComments nalogu. Ograničeno na 30 bedževa.
- `override` - Ako je true, svi postojeći bedževi prikazani na komentarima biće zamenjeni sa navedenim. Ako je false ili izostavljeno, navedeni bedževi će biti dodati postojećim bedževima.
- `update` - Ako je true, osobine prikaza bedževa će biti ažurirane iz konfiguracije tenanta kad god se korisnik prijavi.