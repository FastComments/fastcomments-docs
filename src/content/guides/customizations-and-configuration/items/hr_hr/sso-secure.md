[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO koristi HMAC‑SHA256 enkripciju kao mehanizam za implementaciju SSO. Prvo ćemo pregledati cjelokupnu arhitekturu, pružiti primjere i detaljne korake.

Postoji i dokumentacija o migraciji s drugih pružatelja koji imaju slične SSO mehanizme, te o razlikama.

Tok izgleda ovako:

<div class="screenshot white-bg">
    <div class="title">Siguran SSO tok</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Dijagram sigurnog SSO" />
</div>

Budući da Secure SSO uključuje full‑stack razvoj, potpuni radni primjeri koda u Java/Spring, NodeJS/Express i vanilla PHP trenutno su <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHubu</a>.

Iako koristimo ExpressJS u NodeJS primjeru i Spring u Java primjeru, nema potreba za frameworkovima/bibliotekama u tim runtime‑ovima za implementaciju FastComments SSO – rade nativni crypto paketi.

Ne morate pisati nove API endpointove s FastComments SSO. Jednostavno šifrirajte informacije korisnika koristeći svoj tajni ključ i proslijedite payload widgetu za komentare.

#### Preuzmite svoj API tajni ključ

Vaš API tajni ključ možete preuzeti s <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">ove stranice</a>. Također ovu stranicu možete pronaći odlaskom na Moj račun, klikom na pločicu API/SSO, a zatim klikom na „Preuzmi API tajni ključ“.

#### Parametri widgeta za komentare

Visokorazinska API dokumentacija za widget za komentare može se naći <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">ovdje</a>.

Idemo detaljnije u značenje ovih parametara.

Widget za komentare prima objekt konfiguracije – već ga prosljeđujete ako koristite FastComments za prosljeđivanje ID‑a vašeg kupca (zvanog tenantId).

Za omogućavanje SSO, proslijedite novi „sso“ objekt, koji mora sadržavati sljedeće parametre. Vrijednosti bi trebale biti generirane na serveru.

- userDataJSONBase64: Podaci korisnika u JSON formatu, koji se zatim kodiraju u Base64.
- verificationHash: HMAC‑SHA256 hash kreiran iz UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Epoch vremenski žig, u **milisekundama**. Ne smije biti u budućnosti, niti stariji od dva dana.
- loginURL: URL koji widget za komentare može prikazati za prijavu korisnika.
- logoutURL: URL koji widget za komentare može prikazati za odjavu korisnika.
- loginCallback: Kada se navede umjesto login URL‑a, funkcija koju će widget za komentare pozvati pri kliku na gumb za prijavu.
- logoutCallback: Kada se navede umjesto logout URL‑a, funkcija koju će widget za komentare pozvati pri kliku na gumb za odjavu.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Siguran SSO klijentski kod'; isFunctional = false; code-example-end]

#### Objekt korisnika

[inline-code-attrs-start title = 'Objekt korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Obavezno. Najviše 1k znakova. **/
    id: string;
    /** Obavezno. Najviše 1k znakova. Napomena: Mora biti jedinstveno. **/
    email: string;
    /** Obavezno. Najviše 1k znakova. Napomena: Korisničko ime ne smije biti email. Ne mora biti jedinstveno. **/
    username: string;
    /** Opcionalno. Najviše 3k znakova za URL-ove. Zadano je iz gravatara na temelju emaila. Podržava 64‑kodirane slike, u tom slučaju limit je 50k znakova. **/ 
    avatar?: string;
    /** Opcionalno. Zadano false. **/
    optedInNotifications?: boolean;
    /** Opcionalno. Zadano false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcionalno. Najviše 100 znakova. Ova oznaka će se prikazati uz njihovo ime. Zadano je Administrator/Moderator kada je primjenjivo. **/
    displayLabel?: string;
    /** Opcionalno. Najviše 500 znakova. Ovo će se prikazati umjesto korisničkog imena. **/
    displayName?: string;
    /** Opcionalno. Najviše 2k znakova. Ime korisnika će biti poveznica na ovo. **/
    websiteUrl?: string;
    /** Opcionalno. Do 100 grupa po korisniku. ID grupe ne smije biti duži od 50 znakova. **/
    groupIds?: string[];
    /** Opcionalno. Označava korisnika kao administratora. **/
    isAdmin?: boolean;
    /** Opcionalno. Označava korisnika kao moderatora. **/
    isModerator?: boolean;
    /** Opcionalno, zadano true. Postavite na false da omogućite karticu "aktivnost" u korisničkom profilu. **/
    isProfileActivityPrivate?: boolean;
    /** Opcionalno, zadano false. Postavite na true da onemogućite komentare u profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcionalno, zadano false. Postavite na true da onemogućite izravno slanje poruka ovom korisniku. **/
    isProfileDMDisabled?: boolean;
    /** Opcionalna konfiguracija za značke korisnika. **/
    badgeConfig?: {
        /** Niz globalnih ID‑ova znački za dodjelu. Ograničeno na 30 znački. Redoslijed se poštuje. **/
        badgeIds: string[];
        /** Niz ID‑ova znački ograničenih na trenutnu stranicu (urlId). Prikazuje se samo na dodijeljenoj stranici. **/
        pageBadgeIds?: string[];
        /** Ako je true, zamjenjuje postojeće prikazane značke. Globalne i na stranici ograničene značke zamjenjuju se neovisno. **/
        override?: boolean;
        /** Ako je true, ažurira svojstva prikaza znački iz konfiguracije najmodavca. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatori i administratori

Za administratore i moderatore, proslijedite odgovarajuće `isAdmin` ili `isModerator` zastavice u objektu `SSOUser`.

#### Obavijesti

Za omogućavanje ili onemogućavanje obavijesti, postavite vrijednost `optedInNotifications` na `true` ili `false` prema potrebi. Prvi put kada korisnik učita stranicu s ovom vrijednošću u SSO payloadu, njegove postavke obavijesti bit će ažurirane.

Dodatno, ako želite da korisnici primaju email obavijesti o aktivnostima na stranicama na koje su pretplaćeni (umjesto samo obavijesti u aplikaciji), postavite `optedInSubscriptionNotifications` na `true`.

#### VIP korisnici i posebne oznake

Možete prikazati posebnu oznaku uz ime korisnika koristeći opcionalno polje "displayLabel".

#### Neprijavljeni korisnici

Za predstavljanje neautentificiranog korisnika, jednostavno ne popunite userDataJSONBase64, verificationHash ili timestamp. Dostavite loginURL.

Ti korisnici neće moći komentirati, već će im se prikazati poruka za prijavu (poruka, poveznica ili gumb, ovisno o konfiguraciji).

#### Izravni primjeri za serijalizaciju i hashiranje podataka korisnika

Više detalja i primjeri <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/node-express/routes/index.js#L23" target="_blank">ovdje</a> (js), <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/java-springboot/src/main/java/com/winricklabs/ssodemo/DemoController.java#L68" target="_blank">ovdje</a> (java) i <a href="https://github.com/FastComments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">ovdje</a> (php).

Razumijemo da svaka integracija može biti kompliciran i naporan proces. Ne oklijevajte kontaktirati svog predstavnika ili koristiti <a href="https://fastcomments.com/auth/my-account/help" target="_blank">stranicu za podršku</a>.

---