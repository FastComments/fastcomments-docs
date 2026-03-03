[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC-SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo proći kroz ukupnu arhitekturu, dati primjere i detaljne korake.

Također postoji i dokumentacija koja opisuje migraciju s drugih pružatelja sličnih SSO mehanizama, i razlike.

Tijek izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Tijek sigurnog SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Dijagram sigurnog SSO" />
</div>

Budući da Secure SSO uključuje full-stack razvoj, potpuni radni primjeri koda u Java/Spring, NodeJS/Express i vanilla PHP trenutno su <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Iako koristimo ExpressJS u NodeJS primjeru i Spring u Java primjeru, u tim runtime-ovima nisu potrebni posebni framework-i/biblioteke za implementaciju FastComments SSO - rade nativni crypto paketi.

Ne morate pisati nikakve nove API endpoint-e s FastComments SSO. Jednostavno enkriptirajte informacije o korisniku koristeći svoj tajni ključ i proslijedite payload u comment widget.

#### Dohvatite svoj API tajni ključ

Vaš API tajni ključ možete dohvatiti s <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Ovu stranicu također možete pronaći odlaskom na Moj račun, klikom na pločicu API/SSO, a zatim klikom na "Preuzmi API tajni ključ".

#### Parametri widgeta za komentare

Visokonivodna API dokumentacija za comment widget može se pronaći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovdje</a>.

Idemo detaljnije što ti parametri znače.

Comment widget prima konfiguracijski objekt - već prosljeđujete ovaj objekt ako koristite FastComments za prosljeđivanje vašeg identifikatora kupca (koji se zove tenantId).

Da biste omogućili SSO, proslijedite novi objekt "sso", koji mora imati sljedeće parametre. Vrijednosti bi trebale biti generirane na serverskoj strani.

- userDataJSONBase64: Podaci korisnika u JSON formatu, koji se zatim Base64 enkodiraju.
- verificationHash: HMAC-SHA256 hash kreiran iz UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch timestamp, u **milisekundama**. Ne smije biti u budućnosti niti stariji više od dva dana.
- loginURL: URL koji comment widget može prikazati za prijavu korisnika.
- logoutURL: URL koji comment widget može prikazati za odjavu korisnika.
- loginCallback: Kada je pruženo umjesto login URL-a, funkcija koju će comment widget pozvati pri kliku na gumb za prijavu.
- logoutCallback: Kada je pruženo umjesto logout URL-a, funkcija koju će comment widget pozvati pri kliku na gumb za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

[inline-code-attrs-start title = 'Objekt korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Maksimalno 1k znakova. **/
    id: string;
    /** Obavezno. Maksimalno 1k znakova. Napomena: Mora biti jedinstveno. **/
    email: string;
    /** Obavezno. Maksimalno 1k znakova. Napomena: Korisničko ime ne može biti email. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Maksimalno 3k znakova za URL-ove. Zadano dolazi iz Gravatara na temelju emaila. Podržava 64 kodirane slike, u kojem slučaju ograničenje je 50k znakova. **/ 
    avatar?: string;
    /** Opcionalno. Zadana vrijednost je false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Zadana vrijednost je false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Maksimalno 100 znakova. Ova oznaka će se prikazati pored njihovog imena. Zadano je Administrator/Moderator kad je primjenjivo. **/
    displayLabel?: string;
    /** Opcionalno. Maksimalno 500 znakova. Ovo će se prikazati umjesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Maksimalno 2k znakova. Ime korisnika će voditi na ovu adresu. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne smije biti duži od 50 znakova. **/
    groupIds?: string[];
    /** Opcionalno. Označava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Označava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, zadano true. Postavite na false da biste omogućili karticu 'aktivnost' u profilu korisnika. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, zadano false. Postavite na true da biste onemogućili komentare u profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, zadano false. Postavite na true da biste onemogućili slanje direktnih poruka ovom korisniku. **/
    isProfileDMDisabled?: boolean;
    /** Opcionalna konfiguracija za značke korisnika. **/
    badgeConfig?: {
        /** Niz globalnih ID-ova znački za dodjelu. Ograničeno na 30 znački. Redoslijed se poštuje. **/
        badgeIds: string[];
        /** Niz ID-ova znački ograničenih na trenutnu stranicu (urlId). Prikazuju se samo na dodijeljenoj stranici. **/
        pageBadgeIds?: string[];
        /** Ako je true, zamjenjuje postojeće prikazane značke. Globalne i na stranici ograničene značke nadjačavaju se neovisno. **/
        override?: boolean;
        /** Ako je true, ažurira prikazne osobine znački iz konfiguracije tenanta. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderators and Administrators

Za administratore i moderatore, proslijedite odgovarajuće flagove `isAdmin` ili `isModerator` u `SSOUser` objektu.

#### Notifications

Da biste omogućili ili onemogućili obavijesti, postavite vrijednost `optedInNotifications` na `true` ili `false`. Prvi put kada korisnik učita stranicu s ovom vrijednošću u SSO payloadu, njegove postavke obavijesti bit će ažurirane.

Dodatno, ako želite da korisnici primaju email obavijesti o aktivnostima na stranicama na koje su se pretplatili (za razliku od samo in-app obavijesti), postavite `optedInSubscriptionNotifications` na `true`.

#### VIP Users & Special Labels

Možete prikazati posebnu oznaku pored imena korisnika koristeći opcionalno polje "displayLabel".

#### Neautentificirani korisnici

Za predstavljanje neautentificiranog korisnika, jednostavno ne popunjavajte userDataJSONBase64, verificationHash niti timestamp. Osigurajte loginURL.

Ti korisnici neće moći komentirati, već će im se prikazati poruka za prijavu (poruka, link ili gumb, ovisno o konfiguraciji).

#### Izravni primjeri serijalizacije i hashiranja podataka korisnika

Više detalja i primjeri dostupni su <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">ovdje</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">ovdje</a> (java) i <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovdje</a> (php).

Razumijemo da svaka integracija može biti kompliciran i bolan proces. Ne oklijevajte kontaktirati svog predstavnika ili koristiti <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu podrške</a>.

---