[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO bruger HMAC-SSHA256-kryptering som mekanisme til at implementere SSO. Først gennemgår vi den overordnede arkitektur, giver eksempler og detaljerede trin.

Der findes også dokumentation vedrørende migrering fra andre udbydere med lignende SSO-mekanismer, samt forskellene.

Flowet ser sådan ud:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Da Secure SSO involverer full-stack udvikling, er der i øjeblikket fuldt fungerende kodeeksempler i Java/Spring, NodeJS/Express og vanilla PHP <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">på GitHub</a>.

Selvom vi bruger ExpressJS i NodeJS-eksemplet og Spring i Java-eksemplet, er der ingen frameworks/libraries påkrævet i disse run-times for at implementere FastComments SSO - de native crypto-pakker fungerer.

Du behøver ikke skrive nogen nye API-endpoints med FastComments SSO. Krypter blot brugerens oplysninger med din hemmelige nøgle og send payload'en til comment widget'en.

#### Hent din API Secret Key

Din API Secret kan hentes fra <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">denne side</a>. Du kan også finde denne side ved at gå til My Account, klikke på API/SSO-flisen og derefter klikke på "Get API Secret Key".

#### Comment Widget-parametre

API-dokumentation på højt niveau for comment widget'en kan findes <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">her</a>.

Lad os gå mere i detaljer med, hvad disse parametre betyder.

Comment widget'en tager et konfigurationsobjekt - du sender dette allerede, hvis du bruger FastComments til at sende dit customer id (kaldet tenantId).

For at aktivere SSO skal du sende et nyt "sso"-objekt, som skal have følgende parametre. Værdierne skal genereres på serversiden.

- userDataJSONBase64: Brugerens data i JSON-format, som derefter Base64-enkodes.
- verificationHash: HMAC-SHA256-hashen oprettet fra UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch-tidsstempel, i **millisekunder**. Må ikke være i fremtiden eller mere end to dage i fortiden.
- loginURL: En URL, som comment widget'en kan vise for at logge brugeren ind.
- logoutURL: En URL, som comment widget'en kan vise for at logge brugeren ud.
- loginCallback: Når denne angives i stedet for login-URL'en, en funktion, som comment widget'en kalder, når der klikkes på login-knappen.
- logoutCallback: Når denne angives i stedet for logout-URL'en, en funktion, som comment widget'en kalder, når der klikkes på logout-knappen.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Klientkode'; isFunctional = false; code-example-end]

#### Brugerobjektet

Brugerobjektet indeholder følgende skema:
[inline-code-attrs-start title = 'Brugerobjektet'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Påkrævet. Maks 1k tegn. **/
    id: string;
    /** Påkrævet. Maks 1k tegn. Bemærk: Skal være unik. **/
    email: string;
    /** Påkrævet. Maks 1k tegn. Bemærk: Brugernavnet må ikke være en e-mail. Behøver ikke være unikt. **/
    username: string;
    /** Valgfri. Maks 3k tegn for URL'er. Standard er fra gravatar baseret på e-mail. Understøtter 64-kodede billeder, i hvilket tilfælde grænsen er 50k tegn. **/ 
    avatar?: string;
    /** Valgfri. Standard false. **/
    optedInNotifications?: boolean;
    /** Valgfri. Standard false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Valgfri. Maks 100 tegn. Dette label vises ved siden af deres navn. Standard er Administrator/Moderator når relevant. **/
    displayLabel?: string;
    /** Valgfri. Maks 500 tegn. Dette vises i stedet for brugernavnet. **/
    displayName?: string;
    /** Valgfri. Maks 2k tegn. Brugerens navn vil linke til dette. **/
    websiteUrl?: string;
    /** Valgfri. Op til 100 grupper pr. bruger. Et gruppe-id må ikke være længere end 50 tegn. **/
    groupIds?: string[];
    /** Valgfri. Angiver brugeren som administrator. **/
    isAdmin?: boolean;
    /** Valgfri. Angiver brugeren som moderator. **/
    isModerator?: boolean;
    /** Valgfri, standard true. Sæt til false for at aktivere "activity"-fanen i brugerens profil. **/
    isProfileActivityPrivate?: boolean;
    /** Valgfri, standard false. Sæt til true for at deaktivere profilkommentarer. **/
    isProfileCommentsPrivate?: boolean;
    /** Valgfri, standard false. Sæt til true for at deaktivere direkte beskeder til denne bruger. **/
    isProfileDMDisabled?: boolean;
    /** Valgfri konfiguration for brugerbadges. **/
    badgeConfig?: {
        /** Array af globale badge-id'er, der skal tildeles. Begrænset til 30 badges. Rækkefølgen respekteres. **/
        badgeIds: string[];
        /** Array af badge-id'er afgrænset til den aktuelle side (urlId). Vises kun på den tildelte side. **/
        pageBadgeIds?: string[];
        /** Hvis true, erstatter eksisterende viste badges. Global og sideafgrænset tilsidesættes uafhængigt. **/
        override?: boolean;
        /** Hvis true, opdaterer badge-visningsegenskaber fra tenant-konfigurationen. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatorer og Administratorer

For admins og moderatorer skal du sende de respektive `isAdmin`- eller `isModerator`-flag i `SSOUser`-objektet.

#### Notifikationer

For at aktivere eller deaktivere notifikationer skal du sætte værdien af `optedInNotifications` til henholdsvis `true` eller `false`. Første gang brugeren indlæser siden med denne værdi i SSO-payload'en, opdateres deres notifikationsindstillinger.

Derudover, hvis du ønsker, at brugere skal modtage notifikations-e-mails for aktivitet på sider, de har tilmeldt sig (i modsætning til kun in-app notifikationer), så sæt `optedInSubscriptionNotifications` til `true`.

#### VIP-brugere & Særlige labels

Du kan vise et særligt label ved siden af brugerens navn ved at bruge det valgfrie "displayLabel"-felt.

#### Uautentificerede brugere

For at repræsentere en uautentificeret bruger skal du blot ikke udfylde userDataJSONBase64, verificationHash eller timestamp. Angiv en loginURL.

Disse brugere vil ikke kunne kommentere, og i stedet præsenteres de for en login-besked (besked, link eller knap, afhængigt af konfigurationen).

#### Direkte eksempler på serialisering og hashing af brugerdata

Flere detaljer som eksempler findes <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">her</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">her</a> (java) og <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">her</a> (php).

Vi forstår, at enhver integration kan være en kompliceret og smertefuld proces. Tøv ikke med at kontakte din repræsentant eller bruge <a href="https://fastcomments.com/auth/my-account/help" target="_blank">supportsiden</a>.