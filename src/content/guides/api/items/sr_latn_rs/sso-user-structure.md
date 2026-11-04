FastComments pruža jednostavno rešenje za SSO. Ažuriranje informacija o korisniku putem HMAC-bazirane integracije je
jednostavno kao to da korisnik učita stranicu sa ažuriranim payload-om.

Međutim, može biti poželjno upravljati korisnikom van tog toka, radi bolje konzistentnosti vaše aplikacije.

SSO User API pruža način za CRUD operacije nad objektima koje nazivamo SSOUsers. Ovi objekti su različiti od običnih Users i
održavaju se odvojeno radi sigurnosti tipova.

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
    isAccountOwner?: boolean // Admin dozvola - SSO korisnici sa ovom oznakom se naplaćuju kao SSO Admini (odvojeno od običnih SSO korisnika)
    isAdminAdmin?: boolean // Admin dozvola - SSO korisnici sa ovom oznakom se naplaćuju kao SSO Admini (odvojeno od običnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Dozvola moderatora - SSO korisnici sa ovom oznakom se naplaćuju kao SSO Moderatori (odvojeno od običnih SSO korisnika)
    /** Ako je null, Kontrola pristupa se ne primenjuje na korisnika. Ako je prazna lista, ovaj korisnik neće moći da vidi nijednu stranicu niti da @mention-uje druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dozvolite drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njegovom profilu. Podrazumevano je true radi sigurnih profila. **/
    isProfileActivityPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da ostavljaju komentare na profilu korisnika, niti da vide postojeće komentare profila. Podrazumevano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da šalju direktne poruke ovom korisniku. Podrazumevano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opciona konfiguracija za bedževe korisnika. **/
    badgeConfig?: {
        /** Niz ID-jeva bedževa koje treba dodeliti korisniku. Ograničeno na 30 bedževa. Redosled se poštuje. Ovo su globalni bedževi vidljivi na svim stranicama. **/
        badgeIds: string[]
        /** Niz ID-jeva bedževa ograničenih na trenutnu stranicu (urlId). Ovi bedževi se prikazuju samo na stranici na kojoj su dodeljeni. **/
        pageBadgeIds?: string[]
        /** Ako je true, zamenjuje sve postojeće prikazane bedževe datim. Globalni bedževi i bedževi ograničeni na stranicu se zamenjuju nezavisno. Ako je false, dodaje se na postojeće bedževe. **/
        override?: boolean
        /** Ako je true, ažurira svojstva prikaza bedževa iz konfiguracije tenanta. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata SSO korisnika

SSO korisnici se naplaćuju različito u zavisnosti od oznaka dozvola:

- **Obični SSO korisnici**: Korisnici bez admin ili moderatorskih dozvola naplaćuju se kao obični SSO korisnici
- **SSO Admini**: Korisnici sa oznakama `isAccountOwner` ili `isAdminAdmin` naplaćuju se odvojeno kao SSO Admini (ista stopa kao za obične admine tenanta)
- **SSO Moderatori**: Korisnici sa oznakom `isCommentModeratorAdmin` naplaćuju se odvojeno kao SSO Moderatori (ista stopa kao za obične moderatore)

**Važno**: Da bi se sprečilo dvostruko naplaćivanje, sistem automatski deduplira SSO korisnike u odnosu na obične korisnike i moderatore tenanta prema email adresi. Ako SSO korisnik ima isti email kao običan korisnik ili moderator tenanta, neće se naplaćivati dva puta.

### Kontrola pristupa

Korisnici mogu biti podeljeni u grupe. Za to služi polje `groupIds`, i opciono je.

### @Mentions

Podrazumevano, `@mentions` koriste `username` za pretragu drugih SSO korisnika kada se ukuca znak `@`. Ako se koristi `displayName`, rezultati koji se podudaraju sa `username` biće ignorisani kada postoji podudaranje sa `displayName`, a rezultati pretrage `@mention` koristiće `displayName`.

### Pretplate

U FastComments-u, korisnici mogu da se pretplate na stranicu klikom na ikonu zvonce u vidžetu za komentare i klikom na Subscribe.

Kod običnog korisnika, šaljemo mu email obaveštenja na osnovu njegovih podešavanja obaveštenja.

Kod SSO korisnika, ovo razdvajamo radi unazad kompatibilnosti. Korisnici će primati ove dodatne emailove o pretplati samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Bedževi

Možete dodeliti bedževe SSO korisnicima pomoću svojstva `badgeConfig`. Bedževi su vizuelni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-jeva bedževa koje treba dodeliti korisniku. Ovo su globalni bedževi vidljivi na svim stranicama. Mora biti reč o važećim ID-jevima bedževa kreiranih u vašem FastComments nalogu. Ograničeno na 30 bedževa.
- `pageBadgeIds` - Opcioni niz ID-jeva bedževa ograničenih na trenutnu stranicu (`urlId`). Ovi bedževi se prikazuju samo na stranici na kojoj su dodeljeni. Različite stranice mogu imati različite bedževe ograničene na stranicu za istog korisnika.
- `override` - Ako je true, svi postojeći prikazani bedževi biće zamenjeni datim. Globalni bedževi i bedževi ograničeni na stranicu se zamenjuju nezavisno — zamena globalnih bedževa ne utiče na bedževe ograničene na stranicu, i obrnuto. Ako je false ili izostavljeno, dati bedževi biće dodati na postojeće.
- `update` - Ako je true, svojstva prikaza bedževa biće ažurirana iz konfiguracije tenanta svaki put kada se korisnik prijavi.