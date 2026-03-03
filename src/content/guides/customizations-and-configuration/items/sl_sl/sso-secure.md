[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO uporablja šifriranje HMAC-SHA256 kot mehanizem za izvedbo SSO. Najprej bomo pregledali splošno arhitekturo, dali primere in podrobne korake.

Obstaja tudi dokumentacija glede migracije iz drugih ponudnikov s podobnimi mehanizmi SSO in razlik.

Tok izgleda takole:

<div class="screenshot white-bg">
    <div class="title">Potek varnega SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Diagram varnega SSO" />
</div>

Ker Secure SSO vključuje razvoj polnega sklopa (full-stack), so delujoči primeri kode v Java/Spring, NodeJS/Express in navadnem PHP trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Čeprav v primeru NodeJS uporabljamo ExpressJS in v primeru Jave Spring, v teh okoljih za implementacijo FastComments SSO niso potrebne nobene posebne knjižnice — zadostujejo vgrajeni kriptografski paketi.

Ni vam treba pisati nobenih novih API endpointov za FastComments SSO. Preprosto šifrirajte uporabnikove podatke s svojim skrivnim ključem in pošljite uporabno obremenitev (payload) v komentarni vtičnik.

#### Pridobite svoj API skrivni ključ

Vaš API skrivni ključ lahko pridobite na <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">tej strani</a>. To stran lahko najdete tudi tako, da greste v Moj račun, kliknete ploščico API/SSO in nato kliknete "Get API Secret Key".

#### Parametri komentarnega vtičnika

Visokoravenjska API dokumentacija za komentarni vtičnik je na voljo <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">tukaj</a>.

Pojdimo bolj v podrobnosti, kaj ti parametri pomenijo.

Komentarni vtičnik sprejme konfiguracijski objekt — tega že posredujete, če uporabljate FastComments za posredovanje vašega ID stranke (imenovan tenantId).

Za omogočanje SSO posredujte nov objekt "sso", ki mora vsebovati naslednje parametre. Vrednosti je treba generirati na strežniški strani.

- userDataJSONBase64: Podatki o uporabniku v formatu JSON, ki so nato Base64 kodirani.
- verificationHash: HMAC-SHA256 hash, ustvarjen iz UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epohni čas v **milisekundah**. Ne sme biti v prihodnosti ali starejši od dveh dni.
- loginURL: URL, ki ga lahko komentarni vtičnik prikaže za prijavo uporabnika.
- logoutURL: URL, ki ga lahko komentarni vtičnik prikaže za odjavo uporabnika.
- loginCallback: Ko je zagotovljen namesto login URL, funkcija, ki jo bo komentarni vtičnik poklical ob kliku na gumb za prijavo.
- logoutCallback: Ko je zagotovljen namesto logout URL, funkcija, ki jo bo komentarni vtičnik poklical ob kliku na gumb za odjavo.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Objekt uporabnika

The User object contains the following schema:
[inline-code-attrs-start title = 'Objekt uporabnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obvezno. Največ 1k znakov. **/
    id: string;
    /** Obvezno. Največ 1k znakov. Opomba: Mora biti edinstven. **/
    email: string;
    /** Obvezno. Največ 1k znakov. Opomba: uporabniško ime ne more biti e-poštni naslov. Ni ga treba imeti edinstvenega. **/
    username: string;
    /** Izbirno. Največ 3k znakov za URL-je. Privzeto iz Gravatarja na podlagi e-pošte. Podpira 64-kodirane slike, v tem primeru je omejitev 50k znakov. **/ 
    avatar?: string;
    /** Izbirno. Privzeto false. **/
    optedInNotifications?: boolean;
    /** Izbirno. Privzeto false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Izbirno. Največ 100 znakov. Ta oznaka bo prikazana zraven njihovega imena. Privzeto je Administrator/Moderator, kjer je to primerno. **/
    displayLabel?: string;
    /** Izbirno. Največ 500 znakov. To bo prikazano namesto uporabniškega imena. **/
    displayName?: string;
    /** Izbirno. Največ 2k znakov. Ime uporabnika bo povezano na to. **/
    websiteUrl?: string;
    /** Izbirno. Do 100 skupin na uporabnika. ID skupine ne sme biti daljši od 50 znakov. **/
    groupIds?: string[];
    /** Izbirno. Označuje uporabnika kot administratorja. **/
    isAdmin?: boolean;
    /** Izbirno. Označuje uporabnika kot moderatorja. **/
    isModerator?: boolean;
    /** Izbirno, privzeto true. Nastavite na false, da omogočite zavihek "activity" v uporabnikovem profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Izbirno, privzeto false. Nastavite na true, da onemogočite komentarje na profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Izbirno, privzeto false. Nastavite na true, da onemogočite neposredno pošiljanje sporočil temu uporabniku. **/
    isProfileDMDisabled?: boolean;
    /** Izbirna konfiguracija za značke uporabnika. **/
    badgeConfig?: {
        /** Polje globalnih ID-jev značk za dodelitev. Omejeno na 30 značk. Redosled se upošteva. **/
        badgeIds: string[];
        /** Polje ID-jev značk omejenih na trenutno stran (urlId). Prikazane samo na dodeljeni strani. **/
        pageBadgeIds?: string[];
        /** Če je true, zamenja obstoječe prikazane značke. Globalne in značke omejene na stran se prepišejo neodvisno. **/
        override?: boolean;
        /** Če je true, posodobi lastnosti prikaza značk iz konfiguracije najemnika. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Za skrbnike in moderatorje posredujte ustrezne zastavice `isAdmin` ali `isModerator` v objektu `SSOUser`.

#### Notifications

Če želite omogočiti ali onemogočiti obvestila, nastavite vrednost `optedInNotifications` na `true` ali `false`. Ob prvem nalaganju strani z to vrednostjo v SSO obremenitvi (payload) bodo njihove nastavitve obvestil posodobljene.

Poleg tega, če želite, da uporabniki prejemajo obvestilna e-sporočila za dejavnosti na straneh, na katere so naročeni (namesto le obvestil v aplikaciji), nastavite `optedInSubscriptionNotifications` na `true`.

#### VIP Users & Special Labels

Ob strani uporabnikovega imena lahko prikažete posebno oznako z uporabo izbirnega polja "displayLabel".

#### Unauthenticated users

Za predstavitev neavtoriziranega (neprijavljenega) uporabnika preprosto ne izpolnite userDataJSONBase64, verificationHash ali timestamp. Posredujte loginURL.

Ti uporabniki ne bodo mogli komentirati; namesto tega jim bo prikazano sporočilo za prijavo (sporočilo, povezava ali gumb, odvisno od konfiguracije).

#### Direct Examples for Serializing and Hashing User Data

Več podrobnosti in primerov je <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">tukaj</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">tukaj</a> (java) in <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">tukaj</a> (php).

Zavedamo se, da je kakršnakoli integracija lahko zapleten in naporen proces. Ne oklevajte in se obrnite na svojega predstavnika ali uporabite <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stran za podporo</a>.

---