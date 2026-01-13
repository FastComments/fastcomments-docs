[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC-SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo proći kroz ukupnu arhitekturu, dati primjere i detaljne korake.

Postoji i dokumentacija u vezi migracije s drugih pružatelja usluga koji imaju slične SSO mehanizme, te razlike.

Tijek izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Secure SSO tok</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Dijagram Secure SSO" />
</div>

Budući da Secure SSO uključuje full-stack razvoj, puni radni primjeri koda u Java/Spring, NodeJS/Express i vanilla PHP su trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Iako koristimo ExpressJS u NodeJS primjeru i Spring u Java primjeru, u tim runtime okruženjima nisu potrebne dodatne biblioteke/okviri za implementaciju FastComments SSO - ugrađeni kriptografski paketi su dovoljni.

Ne morate pisati nove API krajnje točke s FastComments SSO. Jednostavno enkriptirajte korisnikove informacije koristeći svoj tajni ključ i proslijedite payload u komentar widget.

#### Nabavite svoj API tajni ključ

Svoj API Secret možete dobiti s <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Ovu stranicu također možete pronaći tako da odete na My Account, kliknete na pločicu API/SSO, a zatim kliknete "Get API Secret Key".

#### Parametri widgeta za komentare

Dokumentacija visokog nivoa za API komentarskog widgeta može se pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovdje</a>.

Idemo detaljnije objasniti što ti parametri znače.

Komentarski widget prima konfiguracijski objekt - već prosljeđujete ovaj objekt ako koristite FastComments za prosljeđivanje vašeg customer id (nazvan tenantId).

Da biste omogućili SSO, proslijedite novi objekt "sso", koji mora sadržavati sljedeće parametre. Vrijednosti bi trebale biti generirane na strani servera.

- userDataJSONBase64: Podaci korisnika u JSON formatu, koji su zatim Base64 kodirani.
- verificationHash: HMAC-SHA256 hash kreiran od UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, u **milisekundama**. Ne smije biti u budućnosti, niti više od dva dana u prošlosti.
- loginURL: URL koji komentarski widget može prikazati za prijavu korisnika.
- logoutURL: URL koji komentarski widget može prikazati za odjavu korisnika.
- loginCallback: Kada je pruženo umjesto login URL-a, funkcija koju će komentarski widget pozvati pri kliku na gumb za prijavu.
- logoutCallback: Kada je pruženo umjesto logout URL-a, funkcija koju će komentarski widget pozvati pri kliku na gumb za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Objekt korisnika

The User object contains the following schema:
[inline-code-attrs-start title = 'Objekt korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Maksimalno 1k znakova. **/
    id: string;
    /** Obavezno. Maksimalno 1k znakova. Napomena: Mora biti jedinstven. **/
    email: string;
    /** Obavezno. Maksimalno 1k znakova. Napomena: Korisničko ime ne može biti e-adresa. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Maks. 3k znakova za URL-ove. Zadano je s gravatara temeljeno na e-pošti. Podržava 64 kodirane slike, u tom slučaju ograničenje je 50k znakova. **/ 
    avatar?: string;
    /** Opcionalno. Zadano false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Zadano false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Maks. 100 znakova. Ovaj će se naziv prikazati uz njihovo ime. Zadano je Administrator/Moderator kada je primjenjivo. **/
    displayLabel?: string;
    /** Opcionalno. Maks. 500 znakova. Ovo će se prikazati umjesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Maks. 2k znakova. Ime korisnika će voditi na ovo. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne smije biti dulji od 50 znakova. **/
    groupIds?: string[];
    /** Opcionalno. Označava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Označava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, zadano true. Postavite na false da omogućite karticu "aktivnost" u korisničkom profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, zadano false. Postavite na true da onemogućite komentare na profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, zadano false. Postavite na true da onemogućite slanje izravnih poruka ovom korisniku. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderatori i administratori

For admins and moderators, pass the respective `isAdmin` or `isModerator` flags in the `SSOUser` object.

#### Obavijesti

To enable or disable notifications, set the value of `optedInNotifications` to `true` or `false` respectively. The first time the user loads the page with this value in the SSO payload, their notification settings will be updated.

Additionally, if you want users to receive notification emails for activity on pages they subscribed to (as opposed to just in-app notifications), then set `optedInSubscriptionNotifications` to `true`.

#### VIP korisnici i posebne oznake

You can display a special label next to the user's name by using the optional "displayLabel" field.

#### Neautentificirani korisnici

To represent an unauthenticated user, simply do not populate userDataJSONBase64, verificationHash, or timestamp. Provide a loginURL.

These users will not be able to comment, and instead will be presented with a login message (message, link, or button, depending on configuration).

#### Izravni primjeri serijalizacije i hashiranja podataka korisnika

More details as an examples <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ovdje</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ovdje</a> (java) and <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovdje</a> (php).

Razumijemo da svaka integracija može biti složen i bolan proces. Ne oklijevajte kontaktirati svog predstavnika ili koristiti <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu za podršku</a>.