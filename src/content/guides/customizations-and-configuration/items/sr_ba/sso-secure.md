[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC-SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo proći kroz ukupnu arhitekturu, dati primjere i detaljne korake.

Postoji i dokumentacija o migraciji sa drugih provajdera sa sličnim SSO mehanizmima, i razlikama.

Tok izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Siguran SSO tok</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Dijagram sigurnog SSO" />
</div>

Budući da Secure SSO uključuje full-stack razvoj, kompletni radni primjeri koda u Java/Spring, NodeJS/Express, i vanilla PHP su trenutno <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Iako koristimo ExpressJS u NodeJS primjeru i Spring u Java primjeru, u ovim runtime-ovima nisu potrebni dodatni framework-i/biblioteke za implementaciju FastComments SSO - ugrađeni kripto paketi rade posao.

Ne morate pisati nove API endpoint-e za FastComments SSO. Jednostavno enkriptujte podatke korisnika koristeći vaš tajni ključ i proslijedite payload u widget za komentare.

#### Get Your API Secret Key

Vaš API Secret se može dobiti sa <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Također ovu stranicu možete pronaći odlaskom na Moj nalog, klikom na API/SSO pločicu, pa potom klikom na "Get API Secret Key".

#### Comment Widget Parameters

Detaljna API dokumentacija za widget za komentare može se naći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovdje</a>.

Detaljnije objasnimo šta ovi parametri znače.

Widget za komentare prima konfiguracijski objekt - već ga prosljeđujete ako koristite FastComments za prosljeđivanje vašeg customer id (zvanog tenantId).

Da biste omogućili SSO, proslijedite novi "sso" objekt, koji mora imati sljedeće parametre. Vrijednosti bi trebalo generisati na serverskoj strani.

- userDataJSONBase64: Podaci korisnika u JSON formatu, koji su potom Base64 kodirani.
- verificationHash: HMAC-SHA256 hash kreiran od UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, u **milisekundama**. Ne smije biti u budućnosti, niti više od dva dana u prošlosti.
- loginURL: URL koji widget za komentare može prikazati za prijavu korisnika.
- logoutURL: URL koji widget za komentare može prikazati za odjavu korisnika.
- loginCallback: Kada je postavljeno umjesto login URL-a, funkcija koju će widget za komentare pozvati prilikom klika na dugme za prijavu.
- logoutCallback: Kada je postavljeno umjesto logout URL-a, funkcija koju će widget za komentare pozvati prilikom klika na dugme za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Objekat korisnika

[inline-code-attrs-start title = 'Objekat korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Maksimalno 1k karaktera. **/
    id: string;
    /** Obavezno. Maksimalno 1k karaktera. Napomena: mora biti jedinstveno. **/
    email: string;
    /** Obavezno. Maksimalno 1k karaktera. Napomena: Korisničko ime ne može biti email. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Maksimalno 3k karaktera za URL-ove. Podrazumijevano dolazi iz gravatar-a na osnovu email-a. Podržava 64 enkodirane slike, u kom slučaju je limit 50k karaktera. **/ 
    avatar?: string;
    /** Opcionalno. Podrazumijevano false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Podrazumijevano false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Maksimalno 100 karaktera. Ova oznaka će biti prikazana pored njihovog imena. Podrazumijevano je Administrator/Moderator kada je primjenjivo. **/
    displayLabel?: string;
    /** Opcionalno. Maksimalno 500 karaktera. Ovo će biti prikazano umjesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Maksimalno 2k karaktera. Ime korisnika će voditi na ovu adresu. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne smije biti duži od 50 karaktera. **/
    groupIds?: string[];
    /** Opcionalno. Oznacava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Oznacava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, podrazumijevano true. Postavite na false da omogućite karticu 'aktivnost' u korisničkom profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, podrazumijevano false. Postavite na true da onemogućite komentare na profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, podrazumijevano false. Postavite na true da onemogućite direktne poruke ovom korisniku. **/
    isProfileDMDisabled?: boolean;
    /** Opcionalna konfiguracija za korisničke značke. **/
    badgeConfig?: {
        /** Niz globalnih ID-eva znački za dodjelu. Ograničeno na 30 znački. Redoslijed se poštuje. **/
        badgeIds: string[];
        /** Niz ID-eva znački ograničenih na trenutnu stranicu (urlId). Prikazuju se samo na dodijeljenoj stranici. **/
        pageBadgeIds?: string[];
        /** Ako je true, zamjenjuje postojeće prikazane značke. Globalne i za stranicu se nadjačavaju nezavisno. **/
        override?: boolean;
        /** Ako je true, ažurira prikaz znački prema konfiguraciji tenanta. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatori i administratori

Za administratore i moderatore, proslijedite odgovarajuće `isAdmin` ili `isModerator` zastavice u `SSOUser` objektu.

#### Notifikacije

Da omogućite ili onemogućite notifikacije, postavite vrijednost `optedInNotifications` na `true` ili `false` respektivno. Prvi put kada korisnik učita stranicu sa ovom vrijednošću u SSO payload-u, njegove postavke notifikacija će biti ažurirane.

Dodatno, ako želite da korisnici primaju email obavijesti za aktivnost na stranicama na koje su se pretplatili (za razliku od samo in-app notifikacija), postavite `optedInSubscriptionNotifications` na `true`.

#### VIP korisnici i posebne oznake

Možete prikazati posebnu oznaku pored imena korisnika koristeći opcionalno polje "displayLabel".

#### Neautentifikovani korisnici

Da predstavljate neautentifikovanog korisnika, jednostavno ne popunjavajte userDataJSONBase64, verificationHash ili timestamp. Obezbijedite loginURL.

Ti korisnici neće moći ostavljati komentare, već će im biti prikazana poruka za prijavu (poruka, link, ili dugme, u zavisnosti od konfiguracije).

#### Direktni primjeri za serijalizaciju i hashiranje podataka korisnika

Više detalja i primjera nalazi se <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ovdje</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ovdje</a> (java) i <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovdje</a> (php).

Razumijemo da svaka integracija može biti složen i bolan proces. Ne oklijevajte kontaktirati vašeg predstavnika ili koristiti <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu podrške</a>.