Objekat `DomainConfig` predstavlja konfiguraciju za domen za zakupca.

Struktura objekta `DomainConfig` je sledeća:

[inline-code-attrs-start title = 'Struktura konfiguracije domena'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domen, ne URL, kao što je "fastcomments.com" ili "www.example.com". Poddomen može biti uključen ako želite ograničiti na poddomen. Maksimalno 1000 karaktera. **/
    domain: string
    /** Ime pošiljaoca koje se koristi prilikom slanja e-poruke. **/
    emailFromName?: string
    /** From-Email koji se koristi pri slanju e-poruke. Osigurajte da je SPF podešen da dozvoli mail.fastcomments.com da šalje e-poruke u ime domena korišćenog u ovom atributu. **/
    emailFromEmail?: string
    /** SAMO ZA ČITANJE. Kada je objekat kreiran. **/
    createdAt: string
    /** Logo povezan sa ovim domenom. Koristi se u e-porukama. Koristite HTTPS. **/
    logoSrc?: string
    /** Manji logo povezan sa ovim domenom. Koristite HTTPS. **/
    logoSrc100px?: string
    /** SAMO SSO. URL koji se koristi u podnožju svake poslate e-poruke. Podržava varijablu "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO SSO. Zaglavlja koja se koriste u svakoj poslatој e-poruci. Korisno, na primer, za postavljanje zaglavlja vezanih za odjavu kako bi se poboljšala isporuka. Unos List-Unsubscribe u ovom zapisu, ako postoji, podržava varijablu "[userId]". **/
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
    /** Selektor DKIM ključa koji treba koristiti. **/
    keySelector: string
    /** Vaš privatni ključ. Počinje sa -----BEGIN PRIVATE KEY----- i završava sa -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Za autentifikaciju

Konfiguracija domena se koristi da odredi koje sajtove mogu da hostuju FastComments widget za vaš nalog. Ovo je osnovni oblik
autentifikacije, što znači da dodavanje ili uklanjanje bilo koje konfiguracije domena može uticati na dostupnost vaše FastComments instalacije
u produkciji.

Ne uklanjajte ili ne ažurirajte `domain` svojstvo `Domain Config` za domen koji je trenutno u upotrebi, osim ako nije namera da se taj domen onemogući.

Ovo ima isto ponašanje kao uklanjanje domena iz [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Takođe imajte na umu da uklanjanje domena iz `My Domains` UI ukloniće svaku odgovarajuću konfiguraciju za taj domen koja je možda dodata putem ovog UI-ja.

### Za prilagođavanje e-pošte

Link za odjavu u podnožju e-poruke, i funkcija jednim klikom za odjavu koju nude mnogi email klijenti, mogu se konfigurisati putem ovog API-ja definisanjem `footerUnsubscribeURL` i `emailHeaders`, redom.

### Za DKIM

Nakon što definišete svoje DKIM DNS zapise, jednostavno ažurirajte DomainConfig sa vašom DKIM konfiguracijom koristeći definisanu strukturu. 

---