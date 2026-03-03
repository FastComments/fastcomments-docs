[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC-SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo proći kroz ukupnu arhitekturu, dati primere i detaljne korake.

Postoji i dokumentacija koja se tiče migracije sa drugih provajdera sa sličnim SSO mehanizmima, i razlika.

Tok izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Secure SSO tok</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Dijagram Secure SSO" />
</div>

Pošto Secure SSO uključuje full-stack razvoj, kompletni radni primeri u Java/Spring, NodeJS/Express, i vanilla PHP su trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHub-u</a>.

Iako koristimo ExpressJS u NodeJS primeru i Spring u Java primeru, nema potrebe za dodatnim framework-ovima/bibliotekama u ovim runtime-ovima da bi se implementirao FastComments SSO - dovoljni su nativni crypto paketi.

Ne morate pisati nikakve nove API endpoint-e sa FastComments SSO. Jednostavno enkriptujte informacije o korisniku koristeći vaš tajni ključ i prosledite payload comment widgetu.

#### Preuzmite svoj API tajni ključ

Vaš API tajni ključ možete preuzeti sa <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Takođe ovu stranicu možete pronaći tako što ćete otići na Moj nalog, kliknuti na API/SSO pločicu, a zatim kliknuti "Preuzmi API tajni ključ".

#### Parametri widgeta za komentare

Visoko-nivo dokumentacije API-ja za widget za komentare može se naći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovde</a>.

Hajde da detaljnije objasnimo šta ovi parametri znače.

Widget za komentare prihvata objekat konfiguracije - već prosleđujete ovo ako koristite FastComments da pošaljete vaš customer id (nazvan tenantId).

Da biste omogućili SSO, prosledite novi "sso" objekat, koji mora imati sledeće parametre. Vrednosti bi trebalo da budu generisane na serverskoj strani.

- userDataJSONBase64: Podaci o korisniku u JSON formatu, koji se zatim Base64 enkodiraju.
- verificationHash: HMAC-SHA256 hash kreiran od UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp u **milisekundama**. Ne sme biti u budućnosti, niti stariji od dva dana.
- loginURL: URL koji widget za komentare može prikazati da bi se korisnik prijavio.
- logoutURL: URL koji widget za komentare može prikazati da bi se korisnik odjavio.
- loginCallback: Kada je obezbeđen umesto login URL-a, funkcija koju će widget za komentare pozvati pri kliku na dugme za prijavu.
- logoutCallback: Kada je obezbeđen umesto logout URL-a, funkcija koju će widget za komentare pozvati pri kliku na dugme za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Objekat korisnika

[inline-code-attrs-start title = 'Objekat korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Maks. 1k karaktera. **/
    id: string;
    /** Obavezno. Maks. 1k karaktera. Napomena: Mora biti jedinstven. **/
    email: string;
    /** Obavezno. Maks. 1k karaktera. Napomena: Korisničko ime ne može biti email. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Maks. 3k karaktera za URL-ove. Podrazumevano iz gravatar-a na osnovu email-a. Podržava 64 enkodirane slike, u kom slučaju je limit 50k karaktera. **/ 
    avatar?: string;
    /** Opcionalno. Podrazumevano false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Podrazumevano false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Maks. 100 karaktera. Ova etiketa će se prikazati pored njihovog imena. Podrazumevano je Administrator/Moderator kada je primenljivo. **/
    displayLabel?: string;
    /** Opcionalno. Maks. 500 karaktera. Ovo će biti prikazano umesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Maks. 2k karaktera. Ime korisnika će linkovati na ovo. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne sme biti duži od 50 karaktera. **/
    groupIds?: string[];
    /** Opcionalno. Označava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Označava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, podrazumevano true. Postavite na false da omogućite karticu "aktivnosti" u profilu korisnika. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, podrazumevano false. Postavite na true da onemogućite komentare u profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, podrazumevano false. Postavite na true da onemogućite direktne poruke tom korisniku. **/
    isProfileDMDisabled?: boolean;
    /** Opcionalna konfiguracija za bedževe korisnika. **/
    badgeConfig?: {
        /** Niz globalnih ID-eva bedževa za dodelu. Ograničeno na 30 bedževa. Redosled se poštuje. **/
        badgeIds: string[];
        /** Niz ID-eva bedževa specifičnih za trenutnu stranicu (urlId). Prikazuju se samo na dodeljenoj stranici. **/
        pageBadgeIds?: string[];
        /** Ako je true, zamenjuje postojeće prikazane bedževe. Globalni i za stranicu se nezavisno prepisuju. **/
        override?: boolean;
        /** Ako je true, ažurira prikazna svojstva bedževa iz tenant konfiguracije. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatori i administratori

Za administratore i moderatore, prosledite odgovarajuće zastavice `isAdmin` ili `isModerator` u `SSOUser` objektu.

#### Obaveštenja

Da biste omogućili ili onemogućili obaveštenja, postavite vrednost `optedInNotifications` na `true` ili `false` respektivno. Prvi put kada korisnik učita stranicu sa ovom vrednošću u SSO payload-u, njegova podešavanja obaveštenja će biti ažurirana.

Dodatno, ako želite da korisnici primaju email obaveštenja za aktivnosti na stranicama na koje su se pretplatili (za razliku od samo in-app obaveštenja), postavite `optedInSubscriptionNotifications` na `true`.

#### VIP korisnici i specijalne oznake

Možete prikazati specijalnu oznaku pored imena korisnika koristeći opciono polje "displayLabel".

#### Neautentifikovani korisnici

Da biste predstavili neautentifikovanog korisnika, jednostavno ne popunjavajte userDataJSONBase64, verificationHash ili timestamp. Obezbedite loginURL.

Ovi korisnici neće moći da komentarišu, već će im se prikazati poruka za prijavu (poruka, link ili dugme, u zavisnosti od konfiguracije).

#### Direktni primeri za serializaciju i heširanje podataka korisnika

Više detalja i primeri možete naći <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ovde</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ovde</a> (java) i <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovde</a> (php).

Razumemo da svaka integracija može biti komplikovan i bolan proces. Ne oklevajte da se obratite svom predstavniku ili koristite <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu za podršku</a>.

---