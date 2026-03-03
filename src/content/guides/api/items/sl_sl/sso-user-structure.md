FastComments zagotavlja enostavno SSO rešitev. Posodabljanje informacij o uporabniku z HMAC-integracijo je tako preprosto, kot da uporabnik naloži stran s posodobljenimi podatki.

Vendar je morda zaželeno upravljati uporabnika zunaj tega poteka, da izboljšate doslednost vaše aplikacije.

SSO User API omogoča način za CRUD (ustvarjanje, branje, posodabljanje, brisanje) objektov, ki jih imenujemo SSOUsers. Ti objekti so drugačni od običajnih Users in so ločeni zaradi varnosti tipov.

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
    isAccountOwner?: boolean // Dovoljenje skrbnika - SSO uporabniki s to zastavico so zaračunani kot SSO skrbniki (ločeno od običajnih SSO uporabnikov)
    isAdminAdmin?: boolean // Dovoljenje skrbnika - SSO uporabniki s to zastavico so zaračunani kot SSO skrbniki (ločeno od običajnih SSO uporabnikov)
    isCommentModeratorAdmin?: boolean // Dovoljenje moderatorja - SSO uporabniki s to zastavico so zaračunani kot SSO moderatorji (ločeno od običajnih SSO uporabnikov)
    /** Če je null, kontrola dostopa ne bo uporabljena za uporabnika. Če je prazna lista, ta uporabnik ne bo mogel videti nobenih strani ali @omeniti drugih uporabnikov. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dovolite drugim uporabnikom videti dejavnosti tega uporabnika, vključno s komentarji, na njegovem profilu. Privzeta vrednost je true, da privzeto zagotavlja varne profile. **/
    isProfileActivityPrivate?: boolean
    /** Ne dovolite drugim uporabnikom puščati komentarjev na uporabnikovem profilu ali videti obstoječih komentarjev na profilu. Privzeto false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dovolite drugim uporabnikom pošiljati direktnih sporočil temu uporabniku. Privzeto false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Neobvezna konfiguracija za značke uporabnika. **/
    badgeConfig?: {
        /** Polje ID-jev značk, ki se dodelijo uporabniku. Omejeno na 30 značk. Redosled se spoštuje. To so globalne značke vidne na vseh straneh. **/
        badgeIds: string[]
        /** Polje ID-jev značk, omejenih na trenutno stran (urlId). Te značke se prikazujejo samo na strani, kjer so bile dodeljene. **/
        pageBadgeIds?: string[]
        /** Če je true, zamenja vse obstoječe prikazane značke s predloženimi. Globalne in stran-specifične značke se preglasijo neodvisno. Če je false, doda k obstoječim značkam. **/
        override?: boolean
        /** Če je true, posodobi lastnosti prikaza značk iz konfiguracije najemnika. **/
        update?: boolean
    }
}
[inline-code-end]

### Zaračunavanje SSO uporabnikov

SSO uporabniki se zaračunavajo različno glede na njihove zastavice dovoljenj:

- **Običajni SSO uporabniki**: uporabniki brez skrbniških ali moderatorskih dovoljenj so zaračunani kot običajni SSO uporabniki
- **SSO skrbniki**: uporabniki z zastavicami `isAccountOwner` ali `isAdminAdmin` so zaračunani ločeno kot SSO skrbniki (enaka tarifa kot običajni skrbniki najemnika)
- **SSO moderatorji**: uporabniki z zastavico `isCommentModeratorAdmin` so zaračunani ločeno kot SSO moderatorji (enaka tarifa kot običajni moderatorji)

**Pomembno**: Za preprečitev dvojnega zaračunavanja sistem samodejno odstrani podvojene SSO uporabnike glede na e-poštni naslov v primerjavi z običajnimi uporabniki najemnika in moderatorji. Če ima SSO uporabnik enak e-poštni naslov kot običajen uporabnik najemnika ali moderator, mu ne bo zaračunano dvakrat.

### Kontrola dostopa

Uporabnike je mogoče razvrstiti v skupine. Za to je polje `groupIds`, in je neobvezno.

### @omenitve

Privzeto bo `@mentions` uporabil `username` za iskanje drugih SSO uporabnikov ob vnosu znaka `@`. Če se uporablja `displayName`, bodo rezultati, ki se ujemajo z `username`, prezrti, kadar obstaja ujemanje za `displayName`, in iskalni rezultati za `@mention` bodo uporabili `displayName`.

### Naročnine

Z FastComments lahko se uporabniki naročijo na stran s klikom na ikono zvonca v komentar widgetu in klikom na Naroči.

Pri običajnem uporabniku jim pošiljamo obvestila po e-pošti glede na njihove nastavitve obvestil.

Pri SSO uporabnikih smo to razdelili zaradi združljivosti z zgodnejšimi različicami. Uporabniki bodo prejeli te dodatne e-pošte o naročninah le, če nastavite `optedInSubscriptionNotifications` na `true`.

### Značke

Z uporabo lastnosti `badgeConfig` lahko dodelite značke SSO uporabnikom. Značke so vizualni indikatorji, ki se pojavijo poleg uporabnikovega imena v komentarjih.

- `badgeIds` - Polje ID-jev značk, ki se dodelijo uporabniku. To so globalne značke, vidne na vseh straneh. Mora biti veljavni ID-ji značk, ustvarjeni v vašem FastComments računu. Omejeno na 30 značk.
- `pageBadgeIds` - Neobvezno polje ID-jev značk, omejenih na trenutno stran (`urlId`). Te značke se prikazujejo samo na strani, kjer so bile dodeljene. Različne strani lahko imajo različne stran-specifične značke za istega uporabnika.
- `override` - Če je true, bodo vse obstoječe prikazane značke zamenjane s predloženimi. Globalne in stran-specifične značke se preglasijo neodvisno — preglasitev globalnih značk ne vpliva na stran-specifične značke, in obratno. Če je false ali izpuščeno, bodo predložene značke dodane k obstoječim.
- `update` - Če je true, bodo lastnosti prikaza značk posodobljene iz konfiguracije najemnika vsakič, ko se uporabnik prijavi.

---