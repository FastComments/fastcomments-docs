[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO bruger HMAC-SHA256-kryptering som mekanismen til at implementere SSO. Først gennemgår vi den overordnede arkitektur, giver eksempler og detaljerede trin.

Der er også noget dokumentation vedrørende migrering fra andre udbydere med lignende SSO-mekanismer, samt forskellene.

Flowet ser sådan ud:

<div class="screenshot white-bg">
    <div class="title">Sikker SSO-flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagram for sikker SSO" />
</div>

Da Secure SSO involverer full-stack-udvikling, er fuldt fungerende kodeeksempler i Java/Spring, NodeJS/Express og vanilla PHP i øjeblikket <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">på GitHub</a>.

Selvom vi bruger ExpressJS i NodeJS-eksemplet og Spring i Java-eksemplet, er der ingen frameworks/biblioteker påkrævet i disse runtime-miljøer for at implementere FastComments SSO - de indbyggede crypto-pakker fungerer fint.

Du behøver ikke at skrive nye API-endpoints med FastComments SSO. Krypter blot brugerens information med din hemmelige nøgle og send payloaden til kommentar-widget'en.

#### Get Your API Secret Key

Din API Secret kan hentes fra <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">denne side</a>. Du kan også finde siden ved at gå til My Account, klikke på API/SSO-flisen og derefter klikke på "Get API Secret Key".

#### Comment Widget Parameters

Overordnet API-dokumentation for kommentar-widgeten kan findes <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">her</a>.

Lad os gå lidt mere i dybden med, hvad disse parametre betyder.

Kommentar-widget'en tager et konfigurationsobjekt - du sender allerede dette, hvis du bruger FastComments til at sende dit kunde-id (kaldet tenantId).

For at aktivere SSO, send et nyt "sso"-objekt, som skal have følgende parametre. Værdierne bør genereres server-side.

- userDataJSONBase64: Brugerens data i JSON-format, som derefter Base64-enkodes.
- verificationHash: HMAC-SHA256-hashen oprettet fra UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch-tidspunkt, i **millisekunder**. Må ikke være i fremtiden, eller mere end to dage i fortiden.
- loginURL: En URL, som kommentar-widget'en kan vise for at logge brugeren ind.
- logoutURL: En URL, som kommentar-widget'en kan vise for at logge brugeren ud.
- loginCallback: Når det leveres i stedet for login-URL'en, en funktion som kommentar-widget'en vil kalde ved klik på login-knappen.
- logoutCallback: Når det leveres i stedet for logout-URL'en, en funktion som kommentar-widget'en vil kalde ved klik på logout-knappen.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

Brugerobjektet indeholder følgende skema:
[inline-code-attrs-start title = 'Brugerobjektet'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Påkrævet. Maks. 1k tegn. **/
    id: string;
    /** Påkrævet. Maks. 1k tegn. Bemærk: Skal være unik. **/
    email: string;
    /** Påkrævet. Maks. 1k tegn. Bemærk: Brugernavnet kan ikke være en e-mail. Behøver ikke være unikt. **/
    username: string;
    /** Valgfri. Maks. 3k tegn for URL'er. Standard er fra gravatar baseret på e-mail. Understøtter base64-kodede billeder, i så fald er grænsen 50k tegn. **/ 
    avatar?: string;
    /** Valgfri. Standard: false. **/
    optedInNotifications?: boolean;
    /** Valgfri. Standard: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Valgfri. Maks. 100 tegn. Denne label vil blive vist ved siden af deres navn. Standard er Administrator/Moderator, hvor relevant. **/
    displayLabel?: string;
    /** Valgfri. Maks. 500 tegn. Dette vises i stedet for brugernavnet. **/
    displayName?: string;
    /** Valgfri. Maks. 2k tegn. Brugerens navn vil linke til dette. **/
    websiteUrl?: string;
    /** Valgfri. Op til 100 grupper per bruger. Et gruppe-id må ikke være længere end 50 tegn. **/
    groupIds?: string[];
    /** Valgfri. Angiver brugeren som administrator. **/
    isAdmin?: boolean;
    /** Valgfri. Angiver brugeren som moderator. **/
    isModerator?: boolean;
    /** Valgfri, standard true. Sæt til false for at aktivere fanen "activity" i brugerens profil. **/
    isProfileActivityPrivate?: boolean;
    /** Valgfri, standard false. Sæt til true for at deaktivere profilkommentarer. **/
    isProfileCommentsPrivate?: boolean;
    /** Valgfri, standard false. Sæt til true for at deaktivere direkte beskeder til denne bruger. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

For administratorer og moderatorer, angiv de respektive `isAdmin` eller `isModerator` flag i `SSOUser`-objektet.

#### Notifications

For at aktivere eller deaktivere notifikationer, sæt værdien af `optedInNotifications` til henholdsvis `true` eller `false`. Første gang brugeren indlæser siden med denne værdi i SSO-payloaden, vil deres notifikationsindstillinger blive opdateret.

Derudover, hvis du ønsker at brugere skal modtage notifikations-e-mails for aktivitet på sider, de har abonneret på (i stedet for kun in-app-notifikationer), så sæt `optedInSubscriptionNotifications` til `true`.

#### VIP Users & Special Labels

Du kan vise en særlig label ved siden af brugerens navn ved at bruge det valgfrie felt "displayLabel".

#### Unauthenticated users

For at repræsentere en uautentificeret bruger, udfyld simpelthen ikke userDataJSONBase64, verificationHash eller timestamp. Angiv en loginURL.

Disse brugere vil ikke kunne kommentere, og i stedet vil de blive præsenteret for en login-besked (besked, link eller knap, afhængigt af konfiguration).

#### Direct Examples for Serializing and Hashing User Data

Flere detaljer og eksempler findes <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">her</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">her</a> (java) og <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">her</a> (php).

Vi forstår, at enhver integration kan være en kompliceret og besværlig proces. Tøv ikke med at kontakte din repræsentant eller bruge <a href="https://fastcomments.com/auth/my-account/help" target="_blank">support-siden</a>.