[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO gebruikt HMAC-SHA256-versleuteling als mechanisme om SSO te implementeren. Eerst bespreken we de algemene architectuur, geven we voorbeelden en gedetailleerde stappen.

Er is ook documentatie beschikbaar over het migreren van andere providers met vergelijkbare SSO-mechanismen, en de verschillen.

De flow ziet er als volgt uit:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Omdat Secure SSO full-stack ontwikkeling omvat, zijn volledig werkende codevoorbeelden in Java/Spring, NodeJS/Express en vanilla PHP momenteel <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">op GitHub</a> te vinden.

Hoewel we ExpressJS gebruiken in het NodeJS-voorbeeld en Spring in het Java-voorbeeld, zijn er geen frameworks/bibliotheken vereist in deze runtimes om FastComments SSO te implementeren - de native crypto-pakketten werken.

Je hoeft geen nieuwe API-endpoints te schrijven met FastComments SSO. Versleutel simpelweg de gebruikersinformatie met je geheime sleutel en geef de payload door aan de comment widget.

#### Verkrijg Je API Secret Key

Je API Secret kan worden opgehaald via <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">deze pagina</a>. Je kunt deze pagina ook vinden door naar My Account te gaan, op de API/SSO-tegel te klikken en vervolgens op "Get API Secret Key" te klikken.

#### Comment Widget Parameters

API-documentatie op hoog niveau voor de comment widget is <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">hier</a> te vinden.

Laten we in meer detail ingaan op wat deze parameters betekenen.

De comment widget neemt een configuratieobject aan - je geeft dit al door als je FastComments gebruikt om je klant-id door te geven (genaamd tenantId).

Om SSO in te schakelen, geef je een nieuw "sso"-object door, dat de volgende parameters moet bevatten. De waarden moeten server-side worden gegenereerd.

- userDataJSONBase64: De gegevens van de gebruiker in JSON-formaat, die vervolgens Base64-gecodeerd zijn.
- verificationHash: De HMAC-SHA256-hash gemaakt van UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch-timestamp, in **milliseconden**. Mag niet in de toekomst liggen, of meer dan twee dagen in het verleden.
- loginURL: Een URL die de comment widget kan tonen om de gebruiker in te loggen.
- logoutURL: Een URL die de comment widget kan tonen om de gebruiker uit te loggen.
- loginCallback: Wanneer dit in plaats van de login-URL wordt meegegeven, een functie die de comment widget aanroept wanneer op de inlogknop wordt geklikt.
- logoutCallback: Wanneer dit in plaats van de logout-URL wordt meegegeven, een functie die de comment widget aanroept wanneer op de uitlogknop wordt geklikt.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Het Gebruikersobject

Het Gebruikersobject bevat het volgende schema:
[inline-code-attrs-start title = 'Het Gebruikersobject'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Required. 1k Characters Max. **/
    id: string;
    /** Required. 1k Characters Max. Note: Must be unique. **/
    email: string;
    /** Required. 1k Characters Max. Note: The username cannot be an email. Does not have to be unique. **/
    username: string;
    /** Optional. 3k Characters Max for URLs. Default is from gravatar based on email. Supports 64 encoded images, in which case the limit is 50k characters. **/ 
    avatar?: string;
    /** Optional. Default false. **/
    optedInNotifications?: boolean;
    /** Optional. Default false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optional. 100 Characters Max. This label will be shown next to their name. Default is Administrator/Moderator when applicable. **/
    displayLabel?: string;
    /** Optional. 500 Characters Max. This will be shown instead of the username. **/
    displayName?: string;
    /** Optional. 2k Characters Max. The user's name will link to this. **/
    websiteUrl?: string;
    /** Optional. Up to 100 groups per user. A group id may not be longer than 50 characters. **/
    groupIds?: string[];
    /** Optional. Denotes the user as an administrator. **/
    isAdmin?: boolean;
    /** Optional. Denotes the user as a moderator. **/
    isModerator?: boolean;
    /** Optional, default true. Set to false to enable the "activity" tab in the user's profile. **/
    isProfileActivityPrivate?: boolean;
    /** Optional, default false. Set to true to disable profile comments. **/
    isProfileCommentsPrivate?: boolean;
    /** Optional, default false. Set to true to disable direct messaging this user. **/
    isProfileDMDisabled?: boolean;
    /** Optional configuration for user badges. **/
    badgeConfig?: {
        /** Array of global badge IDs to assign. Limited to 30 badges. Order is respected. **/
        badgeIds: string[];
        /** Array of badge IDs scoped to the current page (urlId). Only displayed on the assigned page. **/
        pageBadgeIds?: string[];
        /** If true, replaces existing displayed badges. Global and page-scoped are overridden independently. **/
        override?: boolean;
        /** If true, updates badge display properties from tenant configuration. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators en Beheerders

Geef voor admins en moderators de respectievelijke vlaggen `isAdmin` of `isModerator` door in het `SSOUser`-object.

#### Meldingen

Om meldingen in of uit te schakelen, stel je de waarde van `optedInNotifications` respectievelijk in op `true` of `false`. De eerste keer dat de gebruiker de pagina laadt met deze waarde in de SSO-payload, worden hun meldingsinstellingen bijgewerkt.

Als je bovendien wilt dat gebruikers meldingse-mails ontvangen voor activiteit op pagina's waarop ze zijn geabonneerd (in plaats van alleen in-app meldingen), stel je `optedInSubscriptionNotifications` in op `true`.

#### VIP-gebruikers & Speciale Labels

Je kunt een speciaal label naast de naam van de gebruiker tonen door het optionele veld "displayLabel" te gebruiken.

#### Niet-geauthenticeerde gebruikers

Om een niet-geauthenticeerde gebruiker weer te geven, vul je simpelweg userDataJSONBase64, verificationHash of timestamp niet in. Geef een loginURL op.

Deze gebruikers kunnen niet reageren en krijgen in plaats daarvan een inlogbericht te zien (bericht, link of knop, afhankelijk van de configuratie).

#### Directe Voorbeelden voor Serialiseren en Hashen van Gebruikersgegevens

Meer details als voorbeeld zijn <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">hier</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">hier</a> (java) en <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">hier</a> (php) te vinden.

We begrijpen dat elke integratie een gecompliceerd en pijnlijk proces kan zijn. Aarzel niet om contact op te nemen met je vertegenwoordiger of gebruik de <a href="https://fastcomments.com/auth/my-account/help" target="_blank">ondersteuningspagina</a>.