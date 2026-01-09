Objekat `DomainConfig` predstavlja konfiguraciju za domen za zakupca.

Struktura objekta `DomainConfig` je sljedeća:

[inline-code-attrs-start title = 'Struktura konfiguracije domena'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domen, ne URL, poput "fastcomments.com" ili "www.example.com". Poddomen može biti uključen ako je cilj ograničiti na određeni poddomen. Maksimalno 1000 karaktera. **/
    domain: string
    /** Ime pošiljaoca (From-Name) koje se koristi pri slanju e-mailova. **/
    emailFromName?: string
    /** From-Email koji se koristi pri slanju e-mailova. Osigurajte da je SPF podešen da dozvoli mail.fastcomments.com da šalje e-mailove u ime domena koji se koristi u ovom svojstvu. **/
    emailFromEmail?: string
    /** SAMO ZA ČITANJE. Kada je objekat kreiran. **/
    createdAt: string
    /** Logo povezan sa ovim domenom. Koristi se u e-mailovima. Koristite HTTPS. **/
    logoSrc?: string
    /** Manji logo povezan sa ovim domenom. Koristite HTTPS. **/
    logoSrc100px?: string
    /** SAMO SSO. URL koji se koristi u podnožju svakog poslatog e-maila. Podržava varijablu "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO SSO. Zaglavlja koja se koriste u svakom poslatom e-mailu. Korisno, na primjer, za postavljanje zaglavlja vezanih za odjavu kako bi se poboljšala isporuka. Unos List-Unsubscribe u ovom zapisu, ako postoji, podržava varijablu "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Onemogući sve linkove za odjavu. Ne preporučuje se, može negativno uticati na stope isporuke. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM konfiguracija. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura DKIM konfiguracije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Naziv domena u vašem DKIM zapisu. **/
    domainName: string
    /** DKIM selektor ključa koji se koristi. **/
    keySelector: string
    /** Vaš privatni ključ. Počinje sa -----BEGIN PRIVATE KEY----- i završava sa -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Za autentifikaciju

Konfiguracija domena se koristi za određivanje koji sajtovi mogu hostovati FastComments widget za vaš nalog. Ovo je osnovni oblik autentifikacije, što znači da dodavanje ili uklanjanje bilo kojih konfiguracija domena može uticati na dostupnost vaše FastComments instalacije u produkciji.

Ne uklanjajte niti ažurirajte svojstvo `domain` objekta `Domain Config` za domen koji se trenutno koristi, osim ako ne namjeravate onemogućiti taj domen.

Ovo ima isto ponašanje kao uklanjanje domena sa [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Takođe imajte na umu da uklanjanje domena iz `My Domains` UI ukloniće svaku odgovarajuću konfiguraciju za taj domen koja je možda dodata putem tog sučelja.

### Za prilagođavanje e-mailova

Link za odjavu u podnožju e-maila i funkcija jednim klikom za odjavu koju nude mnogi e-mail klijenti mogu se konfigurirati putem ovog API-ja definisanjem `footerUnsubscribeURL` i `emailHeaders`, redom.

### Za DKIM

Nakon što definišete svoje DKIM DNS zapise, jednostavno ažurirajte DomainConfig sa vašom DKIM konfiguracijom koristeći definisanu strukturu.