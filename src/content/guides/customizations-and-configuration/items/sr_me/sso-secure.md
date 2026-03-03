[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC-SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo proći kroz opštu arhitekturu, dati primjere i detaljne korake.

Postoji i dokumentacija u vezi migracije od drugih provajdera sa sličnim SSO mehanizmima, i razlike.

Tok izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Tok sigurnog SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Pošto Secure SSO uključuje full-stack razvoj, potpuni radni primjeri u Java/Spring, NodeJS/Express, i vanilla PHP su trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHub-u</a>.

Iako u NodeJS primjeru koristimo ExpressJS, a u Java primjeru Spring, u ovim runtime-ovima nisu potrebni dodatni framework-i/biblioteke za implementaciju FastComments SSO - ugrađeni kripto paketi su dovoljni.

Ne morate pisati nikakve nove API endpoint-e za FastComments SSO. Jednostavno enkriptujte informacije o korisniku koristeći vaš tajni ključ i proslijedite payload u komentatorski widget.

#### Get Your API Secret Key

Vaš API Secret možete preuzeti sa <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Takođe možete pronaći ovu stranicu tako što ćete otići na My Account, kliknuti API/SSO tile, i zatim kliknuti "Get API Secret Key".

#### Comment Widget Parameters

Visokog nivoa API dokumentacija za komentatorski widget može se naći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovdje</a>.

Hajde da detaljnije objasnimo šta ovi parametri znače.

Komentatorski widget uzima konfiguracioni objekat - vi već prosleđujete ovaj objekat ako koristite FastComments da biste prosledili svoj customer id (zvan tenantId).

Da biste omogućili SSO, prosledite novi "sso" objekat, koji mora imati sljedeće parametre. Vrijednosti treba da budu generisane na serverskoj strani.

- userDataJSONBase64: Podaci korisnika u JSON formatu, koji su zatim Base64 enkodovani.
- verificationHash: HMAC-SHA256 hash kreiran od UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, u **milisekundama**. Ne smije biti u budućnosti, niti više od dva dana u prošlosti.
- loginURL: URL koji komentatorski widget može prikazati za prijavu korisnika.
- logoutURL: URL koji komentatorski widget može prikazati za odjavu korisnika.
- loginCallback: Kada se obezbijedi umjesto login URL-a, funkcija koju će komentatorski widget pozvati pri kliku na dugme za prijavu.
- logoutCallback: Kada se obezbijedi umjesto logout URL-a, funkcija koju će komentatorski widget pozvati pri kliku na dugme za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = 'Objekat korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Maksimalno 1k karaktera. **/
    id: string;
    /** Obavezno. Maksimalno 1k karaktera. Napomena: Mora biti jedinstveno. **/
    email: string;
    /** Obavezno. Maksimalno 1k karaktera. Napomena: Korisničko ime ne može biti email adresa. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Maksimalno 3k karaktera za URL-ove. Podrazumijevano dolazi iz gravatar-a na osnovu email-a. Podržava 64 kodirane slike, u kom slučaju limit je 50k karaktera. **/ 
    avatar?: string;
    /** Opcionalno. Podrazumijevano: false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Podrazumijevano: false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Maksimalno 100 karaktera. Ova oznaka će se prikazati pored njihovog imena. Zadano je Administrator/Moderator kada je primjenjivo. **/
    displayLabel?: string;
    /** Opcionalno. Maksimalno 500 karaktera. Ovo će se prikazati umjesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Maksimalno 2k karaktera. Ime korisnika će voditi na ovo. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne smije biti duži od 50 karaktera. **/
    groupIds?: string[];
    /** Opcionalno. Označava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Označava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, zadano: true. Postavite na false da omogućite tab "activity" u korisničkom profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, zadano: false. Postavite na true da onemogućite komentare na profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, zadano: false. Postavite na true da onemogućite direktno slanje poruka ovom korisniku. **/
    isProfileDMDisabled?: boolean;
    /** Opcionalna konfiguracija za značke korisnika. **/
    badgeConfig?: {
        /** Niz globalnih ID-eva znački za dodjeljivanje. Ograničenje na 30 znački. Redoslijed se poštuje. **/
        badgeIds: string[];
        /** Niz ID-eva znački ograničenih na tekuću stranicu (urlId). Prikazuju se samo na dodijeljenoj stranici. **/
        pageBadgeIds?: string[];
        /** Ako je true, zamjenjuje postojeće prikazane značke. Globalne i na stranici ograničene značke se prepisuju nezavisno. **/
        override?: boolean;
        /** Ako je true, ažurira prikazne osobine znački iz konfiguracije tenanta. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Za admin-e i moderatore, prosledite odgovarajuće `isAdmin` ili `isModerator` zastavice u `SSOUser` objektu.

#### Notifications

Da biste omogućili ili onemogućili notifikacije, postavite vrijednost `optedInNotifications` na `true` ili `false` respektivno. Prvi put kada korisnik učita stranicu sa ovom vrijednošću u SSO payload-u, njegova podešavanja notifikacija će biti ažurirana.

Dodatno, ako želite da korisnici primaju email notifikacije za aktivnost na stranicama koje su pratili (za razliku od samo in-app notifikacija), postavite `optedInSubscriptionNotifications` na `true`.

#### VIP Users & Special Labels

Možete prikazati posebnu oznaku pored korisnikovog imena koristeći opcionalno polje "displayLabel".

#### Unauthenticated users

Da biste predstavili neautentifikovanog korisnika, jednostavno ne popunjavajte userDataJSONBase64, verificationHash, ili timestamp. Obezbijedite loginURL.

Ti korisnici neće moći komentarisati, već će im biti prikazana poruka za prijavu (poruka, link, ili dugme, u zavisnosti od konfiguracije).

#### Direct Examples for Serializing and Hashing User Data

Više detalja i primjeri nalaze se <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ovdje</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ovdje</a> (java) i <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovdje</a> (php).

Razumijemo da bilo koja integracija može biti komplikovan i bolan proces. Ne ustručavajte se da se obratite vašem predstavniku ili iskoristite <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu podrške</a>.

---