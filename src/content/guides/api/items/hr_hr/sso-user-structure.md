FastComments nudi jednostavno rješenje za SSO koje se lako koristi. Ažuriranje korisničkih podataka putem integracije temeljene na HMAC-u jednostavno je kao učitavanje stranice s ažuriranim payloadom od strane korisnika.

Međutim, može biti poželjno upravljati korisnikom izvan tog toka, radi bolje dosljednosti vaše aplikacije.

SSO User API pruža način za CRUD operacije nad objektima koje nazivamo SSOUsers. Ti su objekti različiti od običnih korisnika (Users) i drže se odvojeno radi sigurnosti tipova.

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
    isAccountOwner?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isAdminAdmin?: boolean // Admin permission - SSO users with this flag are billed as SSO Admins (separate from regular SSO users)
    isCommentModeratorAdmin?: boolean // Moderator permission - SSO users with this flag are billed as SSO Moderators (separate from regular SSO users)
    /** If null, Access Control will not be applied to the user. If an empty list, this user will not be able to see any pages or @mention other users. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Don't let other users see this user's activity, including comments, on their profile. Default is true to provide secure profiles by default. **/
    isProfileActivityPrivate?: boolean
    /** Don't let other users leave comments on the user's profile, or see existing profile comments. Default false. **/
    isProfileCommentsPrivate?: boolean
    /** Don't let other users send direct messages to this user. Default false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of badge IDs to assign to the user. Limited to 30 badges. Order is respected. These are global badges visible on all pages. **/
        badgeIds: string[]
        /** Array of badge IDs scoped to the current page (urlId). These badges are only displayed on the page where they were assigned. **/
        pageBadgeIds?: string[]
        /** If true, replaces all existing displayed badges with the provided ones. Global and page-scoped badges are overridden independently. If false, adds to existing badges. **/
        override?: boolean
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean
    }
}
[inline-code-end]

### Naplata za SSO korisnike

SSO korisnici se naplaćuju različito ovisno o njihovim oznakama dozvola:

- **Obični SSO korisnici**: Korisnici bez admin ili moderatorskih dozvola naplaćuju se kao obični SSO korisnici
- **SSO Admini**: Korisnici s oznakama `isAccountOwner` ili `isAdminAdmin` naplaćuju se odvojeno kao SSO Admini (ista cijena kao za obične admin korisnika najmoprimca/tenanta)
- **SSO Moderatori**: Korisnici s oznakom `isCommentModeratorAdmin` naplaćuju se odvojeno kao SSO Moderatori (ista cijena kao za obične moderatore)

**Važno**: Kako bi se spriječila dvostruka naplata, sustav automatski uklanja duplikate SSO korisnika naspram običnih korisnika i moderatora tenanta prema adresi e-pošte. Ako SSO korisnik ima istu e-poštu kao obični korisnik ili moderator tenanta, neće se naplatiti dvaput.

### Kontrola pristupa

Korisnici se mogu podijeliti u grupe. Za to služi polje `groupIds`, i ono je neobavezno.

### @Spominjanja

Prema zadanim postavkama `@mentions` će koristiti `username` za pretraživanje drugih SSO korisnika kada se upiše znak `@`. Ako se koristi `displayName`, tada će rezultati koji odgovaraju `username` biti ignorirani kada postoji podudaranje za `displayName`, a rezultati pretrage `@mention` koristit će `displayName`.

### Pretplate

Uz FastComments, korisnici se mogu pretplatiti na stranicu klikom na ikonu zvona u widgetu za komentare i klikom na Pretplati se.

Kod običnog korisnika, šaljemo im e-poštu s obavijestima na temelju njihovih postavki obavijesti.

Kod SSO korisnika, ovo razdvajamo radi kompatibilnosti unatrag. Korisnici će primati dodatne e-pošte s obavijestima o pretplati samo ako postavite `optedInSubscriptionNotifications` na `true`.

### Značke (Badges)

Možete dodijeliti značke SSO korisnicima koristeći svojstvo `badgeConfig`. Značke su vizualni pokazatelji koji se pojavljuju uz korisničko ime u komentarima.

- `badgeIds` - Polje ID-ova znački koje treba dodijeliti korisniku. To su globalne značke vidljive na svim stranicama. Mora biti riječ o valjanim ID-ovima znački stvorenim u vašem FastComments računu. Ograničeno na 30 znački.
- `pageBadgeIds` - Neobavezno polje ID-ova znački ograničenih na trenutnu stranicu (`urlId`). Te se značke prikazuju samo na stranici na kojoj su dodijeljene. Različite stranice mogu imati različite značke ograničene na stranicu za istog korisnika.
- `override` - Ako je true, sve postojeće prikazane značke bit će zamijenjene pruženima. Globalne i značke ograničene na stranicu nadjačavaju se neovisno — nadjačavanje globalnih znački ne utječe na značke ograničene na stranicu, i obrnuto. Ako je false ili izostavljeno, pružene značke bit će dodane postojećim značkama.
- `update` - Ako je true, svojstva prikaza znački bit će ažurirana iz konfiguracije tenanta svaki put kada se korisnik prijavi.