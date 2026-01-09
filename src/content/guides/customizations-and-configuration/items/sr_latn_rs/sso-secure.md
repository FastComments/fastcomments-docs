[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC-SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo proći kroz celokupnu arhitekturu, dati primere i detaljne korake.

Postoji i dokumentacija o migraciji sa drugih provajdera koji koriste slične SSO mehanizme, i o razlikama.

Tok izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Tok sigurnog SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Dijagram sigurnog SSO" />
</div>

Pošto Secure SSO zahteva full-stack razvoj, kompletni radni primeri u Java/Spring, NodeJS/Express i vanilla PHP su trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHub-u</a>.

Iako koristimo ExpressJS u NodeJS primeru i Spring u Java primeru, u ovim runtime okruženjima nisu potrebni nikakvi dodatni framework-ovi/biblioteke da biste implementirali FastComments SSO - rade native crypto paketi.

Ne morate pisati nikakve nove API endpoint-e za FastComments SSO. Jednostavno enkriptujte korisničke podatke koristeći vaš tajni ključ i prosledite payload comment widget-u.

#### Dobijte svoj API tajni ključ

Vaš API tajni ključ možete preuzeti sa <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Takođe možete doći do ove stranice tako što ćete otići na Moj nalog, kliknuti na API/SSO pločicu, i zatim kliknuti "Get API Secret Key".

#### Parametri komentarskog widgeta

Visok nivo API dokumentacije za comment widget možete pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovde</a>.

Idemo u više detalja šta ovi parametri znače.

Comment widget prihvata konfiguracioni objekat - to već prosleđujete ako koristite FastComments da biste poslali vaš tenant id (nazvan tenantId).

Da biste omogućili SSO, prosledite novi "sso" objekat, koji mora imati sledeće parametre. Vrednosti treba da budu generisane na serverskoj strani.

- userDataJSONBase64: Podaci o korisniku u JSON formatu, koji su zatim Base64 enkodirani.
- verificationHash: HMAC-SHA256 heš napravljen od UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, u **milisekundama**. Ne sme biti u budućnosti, niti stariji više od dva dana.
- loginURL: URL koji comment widget može prikazati za prijavu korisnika.
- logoutURL: URL koji comment widget može prikazati za odjavu korisnika.
- loginCallback: Kada je pruženo umesto login URL-a, funkcija koju će comment widget pozvati pri kliku na dugme za prijavu.
- logoutCallback: Kada je pruženo umesto logout URL-a, funkcija koju će comment widget pozvati pri kliku na dugme za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Objekat korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Maksimalno 1k karaktera. **/
    id: string;
    /** Obavezno. Maksimalno 1k karaktera. Napomena: Mora biti jedinstven. **/
    email: string;
    /** Obavezno. Maksimalno 1k karaktera. Napomena: Korisničko ime ne može biti email. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Maksimalno 3k karaktera za URL-ove. Podrazumevano dolazi od gravatara na osnovu email-a. Podržava base64 enkodirane slike, u kom slučaju je limit 50k karaktera. **/ 
    avatar?: string;
    /** Opcionalno. Podrazumevano false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Podrazumevano false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Maksimalno 100 karaktera. Ova oznaka će biti prikazana pored njihovog imena. Podrazumevano je Administrator/Moderator kada je primenljivo. **/
    displayLabel?: string;
    /** Opcionalno. Maksimalno 500 karaktera. Ovo će biti prikazano umesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Maksimalno 2k karaktera. Ime korisnika će linkovati na ovo. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne sme biti duži od 50 karaktera. **/
    groupIds?: string[];
    /** Opcionalno. Oznacava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Oznacava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, podrazumevano true. Postavite na false da omogućite karticu "activity" u korisnikovom profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, podrazumevano false. Postavite na true da onemogućite komentare na profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, podrazumevano false. Postavite na true da onemogućite direktno slanje poruka ovom korisniku. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderators and Administrators

Za administratore i moderatore, prosledite odgovarajuće `isAdmin` ili `isModerator` flage u `SSOUser` objektu.

#### Notifications

Da biste omogućili ili onemogućili obaveštenja, postavite vrednost `optedInNotifications` na `true` ili `false` respektivno. Prvi put kada se korisnik učita sa ovom vrednošću u SSO payload-u, njegova podešavanja obaveštenja će biti ažurirana.

Dodatno, ako želite da korisnici primaju email notifikacije o aktivnostima na stranicama na koje su pretplaćeni (za razliku od samo in-app notifikacija), postavite `optedInSubscriptionNotifications` na `true`.

#### VIP Users & Special Labels

Možete prikazati specijalnu oznaku pored korisnikovog imena koristeći opciono polje "displayLabel".

#### Unauthenticated users

Da biste predstavili neautentifikovanog korisnika, jednostavno ne popunjavajte userDataJSONBase64, verificationHash, ili timestamp. Obavezno obezbedite loginURL.

Ovi korisnici neće moći da komentarišu, i umesto toga biće im prikazana poruka za prijavu (poruka, link, ili dugme, u zavisnosti od konfiguracije).

#### Direct Examples for Serializing and Hashing User Data

Više detalja i primeri su dostupni <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ovde</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ovde</a> (java) i <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovde</a> (php).

Razumemo da bilo koja integracija može biti komplikovan i bolan proces. Ne oklevajte da kontaktirate vašeg predstavnika ili koristite <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu za podršku</a>.