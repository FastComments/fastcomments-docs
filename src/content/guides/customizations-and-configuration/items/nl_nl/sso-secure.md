[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO gebruikt HMAC-SHA256-encryptie als mechanisme om SSO te implementeren. Eerst bespreken we de algemene architectuur, geven we voorbeelden en gedetailleerde stappen.

Er is ook documentatie over het migreren van andere providers met vergelijkbare SSO-mechanismen, en de verschillen.

De flow ziet er als volgt uit:

<div class="screenshot white-bg">
    <div class="title">Veilige SSO-stroom</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Aangezien Secure SSO full-stack ontwikkeling vereist, staan volledige werkende codevoorbeelden in Java/Spring, NodeJS/Express en vanilla PHP momenteel <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">op GitHub</a>.

Hoewel we ExpressJS in het NodeJS-voorbeeld en Spring in het Java-voorbeeld gebruiken, zijn er geen frameworks/bibliotheken vereist in deze runtimes om FastComments SSO te implementeren — de native crypto-pakketten volstaan.

U hoeft geen nieuwe API-endpoints te schrijven met FastComments SSO. Versleutel eenvoudig de gebruikersgegevens met uw geheime sleutel en geef de payload door aan de comment widget.

#### Get Your API Secret Key

Uw API Secret kunt u ophalen van <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">deze pagina</a>. U kunt deze pagina ook vinden door naar My Account te gaan, op het API/SSO-tegel te klikken en vervolgens op "Get API Secret Key" te klikken.

#### Comment Widget Parameters

High-level API-documentatie voor de comment widget is te vinden <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">hier</a>.

Laten we wat dieper ingaan op wat deze parameters betekenen.

De comment widget neemt een configuratieobject aan — dit geeft u al door als u FastComments gebruikt om uw customer id (tenantId genoemd) mee te geven.

Om SSO in te schakelen, geef een nieuw "sso" object door, dat de volgende parameters moet bevatten. De waarden moeten aan de serverzijde worden gegenereerd.

- userDataJSONBase64: De gebruikersgegevens in JSON-formaat, die vervolgens Base64-geëncodeerd worden.
- verificationHash: De HMAC-SHA256-hash gemaakt van UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch-timestamp, in **milliseconds**. Mag niet in de toekomst liggen, of meer dan twee dagen in het verleden.
- loginURL: Een URL die de comment widget kan tonen om de gebruiker in te loggen.
- logoutURL: Een URL die de comment widget kan tonen om de gebruiker uit te loggen.
- loginCallback: Wanneer opgegeven in plaats van de login URL, een functie die de comment widget zal aanroepen bij het klikken op de login-knop.
- logoutCallback: Wanneer opgegeven in plaats van de logout URL, een functie die de comment widget zal aanroepen bij het klikken op de logout-knop.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

Het volgende schema bevat het User-object:
[inline-code-attrs-start title = 'Het Gebruikersobject'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Vereist. Maximaal 1k tekens. **/
    id: string;
    /** Vereist. Maximaal 1k tekens. Opmerking: moet uniek zijn. **/
    email: string;
    /** Vereist. Maximaal 1k tekens. Opmerking: De gebruikersnaam mag geen e-mailadres zijn. Hoeft niet uniek te zijn. **/
    username: string;
    /** Optioneel. Maximaal 3k tekens voor URL's. Standaard is afkomstig van Gravatar op basis van e-mail. Ondersteunt base64-gecodeerde afbeeldingen, in dat geval is de limiet 50k tekens. **/ 
    avatar?: string;
    /** Optioneel. Standaard false. **/
    optedInNotifications?: boolean;
    /** Optioneel. Standaard false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Optioneel. Maximaal 100 tekens. Dit label wordt naast hun naam weergegeven. Standaard is Administrator/Moderator indien van toepassing. **/
    displayLabel?: string;
    /** Optioneel. Maximaal 500 tekens. Dit wordt weergegeven in plaats van de gebruikersnaam. **/
    displayName?: string;
    /** Optioneel. Maximaal 2k tekens. De naam van de gebruiker zal hiernaar linken. **/
    websiteUrl?: string;
    /** Optioneel. Tot 100 groepen per gebruiker. Een groep-id mag niet langer zijn dan 50 tekens. **/
    groupIds?: string[];
    /** Optioneel. Geeft aan dat de gebruiker een administrator is. **/
    isAdmin?: boolean;
    /** Optioneel. Geeft aan dat de gebruiker een moderator is. **/
    isModerator?: boolean;
    /** Optioneel, standaard true. Zet op false om het tabblad "activiteit" in het profiel van de gebruiker in te schakelen. **/
    isProfileActivityPrivate?: boolean;
    /** Optioneel, standaard false. Zet op true om profielreacties uit te schakelen. **/
    isProfileCommentsPrivate?: boolean;
    /** Optioneel, standaard false. Zet op true om directe berichten naar deze gebruiker uit te schakelen. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

Voor admins en moderators geeft u respectievelijk de `isAdmin` of `isModerator` flags door in het `SSOUser` object.

#### Notifications

Om meldingen in of uit te schakelen, stelt u de waarde van `optedInNotifications` in op `true` of `false`. De eerste keer dat de gebruiker de pagina laadt met deze waarde in de SSO-payload, worden hun meldingsinstellingen bijgewerkt.

Bovendien, als u wilt dat gebruikers e-mailmeldingen ontvangen voor activiteit op pagina's waarop ze zijn geabonneerd (in tegenstelling tot alleen in-app meldingen), zet dan `optedInSubscriptionNotifications` op `true`.

#### VIP Users & Special Labels

U kunt een speciaal label naast de naam van de gebruiker tonen door het optionele veld "displayLabel" te gebruiken.

#### Unauthenticated users

Om een niet-geauthenticeerde gebruiker weer te geven, vult u eenvoudig geen userDataJSONBase64, verificationHash of timestamp in. Geef een loginURL op.

Deze gebruikers kunnen niet reageren, en krijgen in plaats daarvan een inlogbericht te zien (bericht, link of knop, afhankelijk van de configuratie).

#### Direct Examples for Serializing and Hashing User Data

Meer details en voorbeelden staan <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">hier</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">hier</a> (java) en <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">hier</a> (php).

We begrijpen dat elke integratie een gecompliceerd en pijnlijk proces kan zijn. Aarzel niet om contact op te nemen met uw vertegenwoordiger of gebruik te maken van de <a href="https://fastcomments.com/auth/my-account/help" target="_blank">supportpagina</a>.