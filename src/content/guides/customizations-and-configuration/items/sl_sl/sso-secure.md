[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO uporablja HMAC-SHA256 šifriranje kot mehanizem za implementacijo SSO. Najprej bomo predstavili splošno arhitekturo, dali primere in podrobna navodila.

Obstaja tudi nekaj dokumentacije glede migracije iz drugih ponudnikov s podobnimi SSO mehanizmi ter razlik.

Potek izgleda takole:

<div class="screenshot white-bg">
    <div class="title">Potek varnega SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagram varnega SSO" />
</div>

Ker Secure SSO vključuje razvoj polne skladovnice (full-stack), so popolni delujoči primeri kode v Java/Spring, NodeJS/Express in navadnem PHP trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Čeprav v primeru NodeJS uporabljamo ExpressJS in v primeru Java uporabljamo Spring, v teh izvajalnih okoljih za implementacijo FastComments SSO niso potrebne dodatne knjižnice ali ogrodja — zadostujejo vgrajeni kriptografski paketi.

Ni vam treba pisati nobenih novih API končnih točk z FastComments SSO. Preprosto šifrirajte uporabnikove podatke z vašim skrivnim ključem in posredujte vsebino (payload) komentarni komponenti.

#### Pridobite svoj API skrivni ključ

Vaš API skrivni ključ lahko pridobite na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">te strani</a>. Do te strani lahko pridete tudi tako, da v meniju izberete My Account, kliknete ploščico API/SSO in nato kliknete "Pridobi API skrivni ključ".

#### Parametri komentarnega gradnika

Visokonivojska dokumentacija API za komentarni gradnik je na voljo <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">tukaj</a>.

Poglejmo podrobneje, kaj ti parametri pomenijo.

Komentarni gradnik prejme konfiguracijski objekt - tega že posredujete, če uporabljate FastComments za posredovanje vašega ID stranke (imenovan tenantId).

Za omogočanje SSO posredujte nov objekt "sso", ki mora vsebovati naslednje parametre. Vrednosti naj bodo generirane na strežniški strani.

- userDataJSONBase64: Podatki o uporabniku v formatu JSON, ki so nato Base64 kodirani.
- verificationHash: HMAC-SHA256 zgoščenka, ustvarjena iz UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Časovni žig (epoch), v **milisekundah**. Ne sme biti v prihodnosti ali starejši od dveh dni.
- loginURL: URL, ki ga lahko komentarni gradnik prikaže za prijavo uporabnika.
- logoutURL: URL, ki ga lahko komentarni gradnik prikaže za odjavo uporabnika.
- loginCallback: Če je zagotovljen namesto prijavnega URL-ja, funkcija, ki jo bo komentarni gradnik poklical ob kliku na gumb za prijavo.
- logoutCallback: Če je zagotovljen namesto odjave URL-ja, funkcija, ki jo bo komentarni gradnik poklical ob kliku na gumb za odjavo.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Objekt uporabnika

Uporabniški objekt vsebuje naslednjo shemo:
[inline-code-attrs-start title = 'Objekt uporabnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obvezno. Največ 1k znakov. **/
    id: string;
    /** Obvezno. Največ 1k znakov. Opomba: Mora biti edinstven. **/
    email: string;
    /** Obvezno. Največ 1k znakov. Opomba: uporabniško ime ne sme biti e-poštni naslov. Ni ga treba biti edinstveno. **/
    username: string;
    /** Neobvezno. Največ 3k znakov za URL-je. Privzeto iz gravatar glede na e-pošto. Podpira 64 kodirane slike, v tem primeru je omejitev 50k znakov. **/ 
    avatar?: string;
    /** Neobvezno. Privzeto false. **/
    optedInNotifications?: boolean;
    /** Neobvezno. Privzeto false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Neobvezno. Največ 100 znakov. Ta oznaka bo prikazana zraven njihovega imena. Privzeto je Administrator/Moderator, ko je primerno. **/
    displayLabel?: string;
    /** Neobvezno. Največ 500 znakov. To bo prikazano namesto uporabniškega imena. **/
    displayName?: string;
    /** Neobvezno. Največ 2k znakov. Ime uporabnika bo povezano s tem URL-jem. **/
    websiteUrl?: string;
    /** Neobvezno. Do 100 skupin na uporabnika. ID skupine ne sme biti daljši od 50 znakov. **/
    groupIds?: string[];
    /** Neobvezno. Označuje uporabnika kot skrbnika. **/
    isAdmin?: boolean;
    /** Neobvezno. Označuje uporabnika kot moderatorja. **/
    isModerator?: boolean;
    /** Neobvezno, privzeto true. Nastavite na false, da omogočite zavihek "activity" v uporabnikovem profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Neobvezno, privzeto false. Nastavite na true, da onemogočite komentarje na profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Neobvezno, privzeto false. Nastavite na true, da onemogočite neposredna sporočila temu uporabniku. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderatorji in administratorji

Za skrbnike in moderatorje posredujte ustrezne zastavice `isAdmin` ali `isModerator` v objektu `SSOUser`.

#### Obvestila

Za omogočanje ali onemogočanje obvestil nastavite vrednost `optedInNotifications` na `true` ali `false`. Ob prvem nalaganju strani s to vrednostjo v SSO vsebini bodo nastavitve obvestil uporabnika posodobljene.

Poleg tega, če želite, da uporabniki prejemajo e-poštna obvestila za dejavnosti na straneh, na katere so naročeni (namesto samo obvestil v aplikaciji), nastavite `optedInSubscriptionNotifications` na `true`.

#### VIP uporabniki in posebne oznake

Ob uporabnikovem imenu lahko prikažete posebno oznako z uporabo izbirnega polja "displayLabel".

#### Neavtorizirani uporabniki

Za predstavitev neavtoriziranega uporabnika preprosto ne izpolnite userDataJSONBase64, verificationHash ali timestamp. Zagotovite loginURL.

Ti uporabniki ne bodo mogli komentirati; namesto tega jim bo prikazano sporočilo za prijavo (sporočilo, povezava ali gumb, odvisno od konfiguracije).

#### Neposredni primeri za serializacijo in zgoščevanje uporabniških podatkov

Več podrobnosti in primerov je na voljo <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">tukaj</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">tukaj</a> (java) in <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">tukaj</a> (php).

Zavedamo se, da je vsaka integracija lahko zapleten in naporen proces. Ne oklevajte in se obrnite na svojega predstavnika ali uporabite <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stran za podporo</a>.