FastComments ponuja preprosto rešitev SSO za uporabo. Posodobitev informacij o uporabniku s HMAC-based integracijo je
tako preprosta, da uporabnik naloži stran s posodobljenim payloadom.

Vendar je lahko zaželeno upravljati uporabnika zunaj tega toka, da izboljšate doslednost vaše aplikacije.

SSO User API omogoča CRUD operacije nad objekti, ki jih imenujemo SSOUsers. Ti objekti so drugačni od običajnih Users in
so ločeni zaradi varnosti tipov.

Struktura objekta SSOUser je naslednja:

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

### Obračavanje SSO uporabnikov

SSO uporabniki se obračunavajo različno glede na njihove zastavice dovoljenj:

- **Običajni SSO uporabniki**: Uporabniki brez admin ali moderator dovoljenj se obračunajo kot običajni SSO uporabniki
- **SSO Admini**: Uporabniki z zastavicama `isAccountOwner` ali `isAdminAdmin` se obračunajo ločeno kot SSO Admini (ista cena kot običajni admini najemnika)
- **SSO Moderatorji**: Uporabniki z zastavico `isCommentModeratorAdmin` se obračunajo ločeno kot SSO Moderatorji (ista cena kot običajni moderatorji)

**Pomembno**: Za preprečevanje dvojnega obračunavanja sistem samodejno odstrani dvojnike SSO uporabnikov glede na običajne uporabnike in moderatorje najemnika po e-poštnem naslovu. Če ima SSO uporabnik isti e-poštni naslov kot običajni uporabnik ali moderator najemnika, ne bo obračunan dvakrat.

### Nadzor dostopa

Uporabnike je mogoče razdeliti v skupine. Za to je namenjeno polje `groupIds`, ki je neobvezno.

### @Omembe

Privzeto `@mentions` uporabijo `username` za iskanje drugih sso uporabnikov, ko je vnesen znak `@`. Če je uporabljen `displayName`, bodo rezultati, ki se ujemajo z `username`, prezrti, ko obstaja ujemanje za `displayName`, rezultati iskanja `@mention` pa bodo uporabili `displayName`.

### Naročnine

S FastComments se uporabniki lahko naročijo na stran s klikom na ikono zvončka v pripomočku za komentarje in klikom na Naroči se.

Pri običajnem uporabniku jim pošiljamo obvestilne e-pošte na podlagi njihovih nastavitev obvestil.

Pri SSO uporabnikih smo to razdelili zaradi združljivosti za nazaj. Uporabniki bodo prejeli te dodatne obvestilne e-pošte o naročnini
samo, če nastavite `optedInSubscriptionNotifications` na `true`.

### Značke

SSO uporabnikom lahko dodelite značke z lastnostjo `badgeConfig`. Značke so vizualni kazalniki, ki se prikažejo poleg imena uporabnika v komentarjih.

- `badgeIds` - Polje ID-jev značk za dodelitev uporabniku. To so globalne značke, vidne na vseh straneh. Morajo biti veljavni ID-ji značk, ustvarjenih v vašem FastComments računu. Omejeno na 30 značk.
- `pageBadgeIds` - Neobvezno polje ID-jev značk, obseženih na trenutno stran (`urlId`). Te značke so prikazane samo na strani, kjer so bile dodeljene. Različne strani lahko imajo različne značke obsežene na stran za istega uporabnika.
- `override` - Če je true, bodo vse obstoječe prikazane značke zamenjane s podanimi. Globalne in na stran obsežene značke so preglasene neodvisno — preglaševanje globalnih značk ne vpliva na značke obsežene na stran in obratno. Če je false ali izpuščeno, bodo podane značke dodane obstoječim značkam.
- `update` - Če je true, bodo lastnosti prikaza značk posodobljene iz konfiguracije najemnika vsakič, ko se uporabnik prijavi.