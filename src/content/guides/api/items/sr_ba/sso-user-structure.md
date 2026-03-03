FastComments pruža jednostavno SSO rješenje. Ažuriranje informacija o korisniku koristeći HMAC-baziranu integraciju je
jednako jednostavno kao da korisnik učita stranicu sa ažuriranim payload-om.

Međutim, može biti poželjno upravljati korisnikom izvan tog toka, radi poboljšanja konzistentnosti vaše aplikacije.

SSO User API pruža način za CRUD objekata koje nazivamo SSOUsers. Ovi objekti se razlikuju od običnih Users i
drže se odvojeno radi tip-sigurnosti.

Struktura objekta SSOUser je sljedeća:

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
    isAccountOwner?: boolean // Dozvola administratora - SSO korisnici sa ovom oznakom se naplaćuju kao SSO administratori (odvojeno od običnih SSO korisnika)
    isAdminAdmin?: boolean // Dozvola administratora - SSO korisnici sa ovom oznakom se naplaćuju kao SSO administratori (odvojeno od običnih SSO korisnika)
    isCommentModeratorAdmin?: boolean // Dozvola moderatora - SSO korisnici sa ovom oznakom se naplaćuju kao SSO moderatori (odvojeno od običnih SSO korisnika)
    /** Ako je null, Kontrola pristupa se neće primjenjivati na korisnika. Ako je prazna lista, ovaj korisnik neće moći vidjeti nikakve stranice niti @mention-ovati druge korisnike. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dozvolite drugim korisnicima da vide aktivnost ovog korisnika, uključujući komentare, na njihovom profilu. Podrazumijevano je true kako bi profili bili sigurni po defaultu. **/
    isProfileActivityPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da ostavljaju komentare na korisnikovom profilu, niti da vide postojeće komentare profila. Podrazumijevano false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dozvolite drugim korisnicima da šalju direktne poruke ovom korisniku. Podrazumijevano false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcionalna konfiguracija za značke korisnika. **/
    badgeConfig?: {
        /** Niz ID-eva znački koje se dodjeljuju korisniku. Ograničeno na 30 znački. Redoslijed se poštuje. Ovo su globalne značke vidljive na svim stranicama. **/
        badgeIds: string[]
        /** Niz ID-eva znački ograničenih na trenutnu stranicu (urlId). Ove značke se prikazuju samo na stranici na kojoj su dodijeljene. **/
        pageBadgeIds?: string[]
        /** Ako je true, zamjenjuje sve postojeće prikazane značke sa datim. Globalne i značke ograničene na stranicu se nezavisno prepisuju. Ako je false, dodaje se na postojeće značke. **/
        override?: boolean
        /** Ako je true, ažurira prikazne osobine znački iz konfiguracije tenant-a. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju različito na osnovu njihovih dozvola:

- **Obični SSO korisnici**: Korisnici bez administratorskih ili moderatorskih dozvola se naplaćuju kao obični SSO korisnici
- **SSO administratori**: Korisnici sa oznakama `isAccountOwner` ili `isAdminAdmin` se naplaćuju odvojeno kao SSO administratori (ista stopa kao obični tenant administratori)
- **SSO moderatori**: Korisnici sa oznakom `isCommentModeratorAdmin` se naplaćuju odvojeno kao SSO moderatori (ista stopa kao obični moderatori)

**Važno**: Da bi se spriječilo dvostruko naplaćivanje, sistem automatski deduplikira SSO korisnike u odnosu na obične tenant korisnike i moderatore po email adresi. Ako SSO korisnik ima isti email kao obični tenant korisnik ili moderator, neće biti naplaćeni dvaput.

### Kontrola pristupa

Korisnici se mogu podijeliti u grupe. Za to služi polje `groupIds`, i ono je opciono.

### @Mentions

Po defaultu `@mentions` će koristiti `username` za pretragu drugih sso korisnika kada se unese karakter `@`. Ako se koristi `displayName`, tada će rezultati koji odgovaraju
`username` biti zanemareni kada postoji podudaranje za `displayName`, i rezultati pretrage za `@mention` će koristiti `displayName`.

### Pretplate

Sa FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widgetu za komentare i klikom na Subscribe.

Kod običnog korisnika, šaljemo im obavijesne emailove u skladu sa njihovim podešavanjima obavijesti.

Kod SSO korisnika, ovo smo razdvojili radi kompatibilnosti unazad. Korisnici će biti poslani ovi dodatni emailovi za obavijesti o pretplati samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Značke

Možete dodijeliti značke SSO korisnicima koristeći svojstvo `badgeConfig`. Značke su vizuelni indikatori koji se pojavljuju pored imena korisnika u komentarima.

- `badgeIds` - Niz ID-eva znački koje se dodjeljuju korisniku. Ovo su globalne značke vidljive na svim stranicama. Moraju biti važeći ID-evi znački kreirani na vašem FastComments nalogu. Ograničeno na 30 znački.
- `pageBadgeIds` - Opcionalni niz ID-eva znački ograničenih na trenutnu stranicu (`urlId`). Ove značke se prikazuju samo na stranici na kojoj su dodijeljene. Različite stranice mogu imati različite značke ograničene na stranicu za istog korisnika.
- `override` - Ako je true, sve postojeće prikazane značke će biti zamijenjene sa datim. Globalne i značke ograničene na stranicu se nezavisno prepisuju — prepisivanje globalnih znački ne utiče na značke ograničene na stranicu, i obrnuto. Ako je false ili izostavljeno, date značke će biti dodate postojećim značkama.
- `update` - Ako je true, prikazne osobine znački će biti ažurirane iz konfiguracije tenant-a svaki put kada se korisnik prijavi.