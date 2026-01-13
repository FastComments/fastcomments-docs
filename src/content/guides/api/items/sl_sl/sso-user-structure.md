FastComments zagotavlja enostavno rešitev SSO. Posodabljanje informacij o uporabniku z integracijo, ki temelji na HMAC, je tako preprosto, kot če uporabnik naloži stran s posodobljenim podatkovnim paketom.

Vendar je včasih zaželeno upravljati z uporabnikom izven tega toka, da izboljšate doslednost vaše aplikacije.

SSO User API omogoča ustvarjanje, branje, posodabljanje in brisanje objektov, ki jim pravimo SSOUsers. Ti objekti se razlikujejo od običajnih Users in so ločeni zaradi tipne varnosti.

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
    isAccountOwner?: boolean // Skrbniška pravica - SSO uporabniki s to zastavico so zaračunani kot SSO Admini (ločeno od običajnih SSO uporabnikov)
    isAdminAdmin?: boolean // Skrbniška pravica - SSO uporabniki s to zastavico so zaračunani kot SSO Admini (ločeno od običajnih SSO uporabnikov)
    isCommentModeratorAdmin?: boolean // Moderatorska pravica - SSO uporabniki s to zastavico so zaračunani kot SSO Moderatorji (ločeno od običajnih SSO uporabnikov)
    /** Če je null, Access Control ne bo uporabljen za uporabnika. Če je seznam prazen, ta uporabnik ne bo mogel videti nobenih strani ali @omeniti drugih uporabnikov. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Ne dovolite drugim uporabnikom videti dejavnosti tega uporabnika, vključno s komentarji, na njegovem profilu. Privzeto je true, da privzeto zagotavlja varne profile. **/
    isProfileActivityPrivate?: boolean
    /** Ne dovolite drugim uporabnikom puščati komentarjev na uporabnikovem profilu ali videti obstoječih komentarjev profila. Privzeto false. **/
    isProfileCommentsPrivate?: boolean
    /** Ne dovolite drugim uporabnikom pošiljati neposrednih sporočil temu uporabniku. Privzeto false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Neobvezna konfiguracija za značke uporabnika. **/
    badgeConfig?: {
        /** Polje ID-jev značk za dodelitev uporabniku. Omejeno na 30 značk. Red je ohranjen. **/
        badgeIds: string[]
        /** Če je true, zamenja vse obstoječe prikazane značke s podanimi. Če je false ali izpuščeno, doda podane značke k obstoječim. **/
        override?: boolean
        /** Če je true, posodobi lastnosti prikaza značk iz konfiguracije najemnika. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO uporabniki se zaračunavajo različno glede na njihove zastavice dovoljenj:

- **Regular SSO Users**: Uporabniki brez skrbniških ali moderatorskih dovoljenj se zaračunavajo kot običajni SSO uporabniki
- **SSO Admins**: Uporabniki z zastavicami `isAccountOwner` ali `isAdminAdmin` se zaračunavajo ločeno kot SSO Admini (enaka stopnja kot običajni najemniški skrbniki)
- **SSO Moderators**: Uporabniki z zastavico `isCommentModeratorAdmin` se zaračunavajo ločeno kot SSO Moderatorji (enaka stopnja kot običajni moderatorji)

**Pomembno**: Da bi preprečili dvojno zaračunavanje, sistem samodejno odstrani podvojene SSO uporabnike glede na običajne najemniške uporabnike in moderatorje po e-poštnem naslovu. Če ima SSO uporabnik isti e-poštni naslov kot običajni najemniški uporabnik ali moderator, ne bo zaračunan dvakrat.

### Access Control

Uporabnike je mogoče razdeliti v skupine. Za to služi polje `groupIds` in je neobvezno.

### @Mentions

Privzeto bodo `@mentions` uporabili `username` za iskanje drugih SSO uporabnikov, ko je vnesen znak `@`. Če se uporablja `displayName`, bodo rezultati, ki se ujemajo z `username`, prezrti, kadar obstaja ujemanje za `displayName`, in rezultati iskanja @omenjanja bodo uporabili `displayName`.

### Subscriptions

Pri FastComments se lahko uporabniki naročijo na stran tako, da kliknejo ikono zvonca v komentarni vtičnik in izberejo Subscribe.

Pri običajnem uporabniku jim pošljemo obvestilna e-poštna sporočila glede na njihove nastavitve obvestil.

Pri SSO uporabnikih to razdelimo zaradi združljivosti nazaj. Uporabniki bodo prejeli ta dodatna obvestilna e-poštna sporočila o naročilih le, če nastavite `optedInSubscriptionNotifications` na `true`.

### Badges

Značke lahko dodelite SSO uporabnikom z uporabo lastnosti `badgeConfig`. Značke so vizualni pokazatelji, ki se prikažejo zraven imena uporabnika pri komentarjih.

- `badgeIds` - Polje ID-jev značk, ki jih je treba dodeliti uporabniku. Ti morajo biti veljavni ID-ji značk, ustvarjeni v vašem FastComments računu. Omejeno na 30 značk.
- `override` - Če je true, bodo vse obstoječe značke, prikazane pri komentarjih, zamenjane s podanimi. Če je false ali izpuščeno, bodo podane značke dodane k obstoječim značkam.
- `update` - Če je true, bodo lastnosti prikaza značk posodobljene iz konfiguracije najemnika vsakič, ko se uporabnik prijavi.

---