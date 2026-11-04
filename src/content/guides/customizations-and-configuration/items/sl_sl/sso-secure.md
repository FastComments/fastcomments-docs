[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO uporablja HMAC-SHA256 šifriranje kot mehanizem za implementacijo SSO. Najprej bomo predstavili splošno arhitekturo, nato primere in podrobne korake.

Obstaja tudi dokumentacija o migraciji od drugih ponudnikov s podobnimi SSO mehanizmi ter o razlikah.

Potek izgleda takole:

<div class="screenshot white-bg">
    <div class="title">Potek Secure SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagram Secure SSO" />
</div>

Ker Secure SSO vključuje razvoj celotnega sklada (full-stack), so popolni delujoči primeri kode v Java/Spring, NodeJS/Express in navadnem PHP trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Čeprav v primeru NodeJS uporabljamo ExpressJS in v primeru Java Spring, za implementacijo FastComments SSO v teh izvajalnih okoljih niso potrebni nobeni frameworki/knjižnice - delujejo izvorne crypto paketi.

S FastComments SSO vam ni treba pisati novih API endpointov. Preprosto šifrirajte podatke uporabnika s svojim skrivnim ključem in podatek (payload) posredujte widgetu za komentarje.

#### Pridobite svoj API skrivni ključ

Svoj API skrivni ključ lahko pridobite na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">tej strani</a>. To stran najdete tudi tako, da odprete Moj račun, kliknete ploščico API/SSO in nato kliknete "Get API Secret Key".

#### Parametri widgeta za komentarje

API dokumentacija widgeta za komentarje na visoki ravni je na voljo <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">tukaj</a>.

Poglejmo podrobneje, kaj ti parametri pomenijo.

Widget za komentarje sprejme konfiguracijski objekt - ta že posredujete, če uporabljate FastComments in podate svoj ID stranke (imenovan tenantId).

Za omogočitev SSO posredujte nov objekt "sso", ki mora imeti naslednje parametre. Vrednosti je treba generirati na strežniški strani.

- userDataJSONBase64: Podatki uporabnika v JSON formatu, ki so nato kodirani v Base64.
- verificationHash: HMAC-SHA256 hash, ustvarjen iz UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch časovni žig, v **milisekundah**. Ne sme biti v prihodnosti ali več kot dva dni v preteklosti.
- loginURL: URL, ki ga widget za komentarje lahko prikaže za prijavo uporabnika.
- logoutURL: URL, ki ga widget za komentarje lahko prikaže za odjavo uporabnika.
- loginCallback: Če je podan namesto URL-ja za prijavo, funkcija, ki jo widget za komentarje pokliče ob kliku na gumb za prijavo.
- logoutCallback: Če je podan namesto URL-ja za odjavo, funkcija, ki jo widget za komentarje pokliče ob kliku na gumb za odjavo.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Odjemalska koda Secure SSO'; isFunctional = false; code-example-end]

#### Objekt User

Objekt User vsebuje naslednjo shemo:
[inline-code-attrs-start title = 'Objekt User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obvezno. Največ 1k znakov. **/
    id: string;
    /** Obvezno. Največ 1k znakov. Opomba: Morala biti unikatna. **/
    email: string;
    /** Obvezno. Največ 1k znakov. Opomba: Uporabniško ime ne more biti e-poštni naslov. Ni nujno, da je unikatno. **/
    username: string;
    /** Neobvezno. Največ 3k znakov za URL-je. Privzeto se uporabi gravatar na podlagi e-pošte. Podpira 64 kodirane slike, v tem primeru je omejitev 50k znakov. **/ 
    avatar?: string;
    /** Neobvezno. Privzeto false. **/
    optedInNotifications?: boolean;
    /** Neobvezno. Privzeto false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Neobvezno. Največ 100 znakov. Ta oznaka bo prikazana poleg imena. Privzeto Administrator/Moderator, kadar je to primerno. **/
    displayLabel?: string;
    /** Neobvezno. Največ 500 znakov. To bo prikazano namesto uporabniškega imena. **/
    displayName?: string;
    /** Neobvezno. Največ 2k znakov. Ime uporabnika bo povezano na to. **/
    websiteUrl?: string;
    /** Neobvezno. Do 100 skupin na uporabnika. ID skupine ne sme biti daljši od 50 znakov. **/
    groupIds?: string[];
    /** Neobvezno. Označi uporabnika kot administratorja. **/
    isAdmin?: boolean;
    /** Neobvezno. Označi uporabnika kot moderatorja. **/
    isModerator?: boolean;
    /** Neobvezno, privzeto true. Nastavite na false, da omogočite zavihek "activity" v profilu uporabnika. **/
    isProfileActivityPrivate?: boolean;
    /** Neobvezno, privzeto false. Nastavite na true, da onemogočite komentarje profila. **/
    isProfileCommentsPrivate?: boolean;
    /** Neobvezno, privzeto false. Nastavite na true, da onemogočite neposredna sporočila temu uporabniku. **/
    isProfileDMDisabled?: boolean;
    /** Neobvezna konfiguracija za značke uporabnika. **/
    badgeConfig?: {
        /** Polje globalnih ID-jev značk za dodelitev. Omejeno na 30 značk. Vrstni red je upoštevan. **/
        badgeIds: string[];
        /** Polje ID-jev značk, obseženih na trenutno stran (urlId). Prikazane so samo na dodeljeni strani. **/
        pageBadgeIds?: string[];
        /** Če je true, zamenja obstoječe prikazane značke. Globalne in stransko obsežene so prepisane neodvisno. **/
        override?: boolean;
        /** Če je true, posodobi lastnosti prikaza značk iz konfiguracije najemnika. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatorji in administratorji

Za administratorje in moderatorje posredujte ustrezni zastavici `isAdmin` ali `isModerator` v objektu `SSOUser`.

#### Obvestila

Za omogočitev ali onemogočitev obvestil nastavite vrednost `optedInNotifications` na `true` oz. `false`. Ko uporabnik prvič naloži stran s to vrednostjo v SSO podatku, bodo njegove nastavitve obvestil posodobljene.

Poleg tega, če želite, da uporabniki prejemajo e-poštna obvestila o aktivnosti na straneh, na katere so naročeni (namesto samo obvestil v aplikaciji), nastavite `optedInSubscriptionNotifications` na `true`.

#### VIP uporabniki & posebne oznake

Poleg imena uporabnika lahko prikažete posebno oznako z uporabo neobveznega polja "displayLabel".

#### Neprijavljeni uporabniki

Za predstavitev neprijavljenega uporabnika preprosto ne izpolnite userDataJSONBase64, verificationHash ali timestamp. Podajte loginURL.

Ti uporabniki ne bodo mogli komentirati, namesto tega jim bo prikazano sporočilo za prijavo (sporočilo, povezava ali gumb, odvisno od konfiguracije).

#### Neposredni primeri za serializacijo in hashiranje podatkov uporabnika

Več podrobnosti kot primeri najdete <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">tukaj</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">tukaj</a> (java) in <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">tukaj</a> (php).

Zavedamo se, da je lahko vsaka integracija zapleten in boleč proces. Ne oklevajte in se obrnite na svojega predstavnika ali uporabite <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stran za podporo</a>.