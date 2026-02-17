Objekt `DomainConfig` predstavlja konfiguraciju za domenu za zakupnika.

Struktura objekta `DomainConfig` izgleda ovako:

[inline-code-attrs-start title = 'Struktura konfiguracije domene'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domena, ne URL, poput "fastcomments.com" ili "www.example.com". Poddomena može biti uključena ako se želi ograničiti na poddomenu. Maksimalno 1000 znakova. **/
    domain: string
    /** Ime pošiljatelja koje se koristi pri slanju e-pošte. **/
    emailFromName?: string
    /** From-Email koji se koristi pri slanju e-pošte. Osigurajte da je SPF postavljen kako bi mail.fastcomments.com mogao slati e-poštu u ime domene korištene u ovom atributu. **/
    emailFromEmail?: string
    /** SAMO ZA ČITANJE. Kada je objekt kreiran. **/
    createdAt: string
    /** Logo povezan s ovom domenom. Koristi se u e-pošti. Koristite HTTPS. **/
    logoSrc?: string
    /** Manji logo povezan s ovom domenom. Koristite HTTPS. **/
    logoSrc100px?: string
    /** SAMO SSO. URL koji se koristi u podnožju svakog poslanog e-maila. Podržava varijablu "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO SSO. Zaglavlja koja se koriste u svakom poslanom e-mailu. Korisno, na primjer, za postavljanje zaglavlja vezanih uz odjavu kako bi se poboljšala isporuka. Unos List-Unsubscribe u ovom zapisu, ako postoji, podržava varijablu "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Onemogući sve poveznice za odjavu. Ne preporučuje se, može smanjiti stopu isporuke. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM konfiguracija. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura DKIM konfiguracije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Naziv domene u vašem DKIM zapisu. **/
    domainName: string
    /** Selektor DKIM ključa koji će se koristiti. **/
    keySelector: string
    /** Javni ključ u PEM formatu. Vraća se u GET odgovorima. **/
    publicKey: string
    /** @deprecated Više se ne vraća u API odgovorima. Pri upisu se prihvaća radi kompatibilnosti unatrag. **/
    privateKey?: string
}
[inline-code-end]

### Za autentikaciju

Konfiguracija domene koristi se za određivanje koje web stranice mogu ugostiti FastComments widget za vaš račun. Ovo je osnovni oblik autentikacije, što znači da dodavanje ili uklanjanje bilo koje konfiguracije domene može utjecati na dostupnost vaše FastComments instalacije u produkciji.

Ne uklanjajte niti ne ažurirajte svojstvo `domain` u `Domain Config` za domenu koja je trenutno u upotrebi osim ako nije namjera onemogućiti tu domenu.

Ovo ima isto ponašanje kao uklanjanje domene iz [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Također imajte na umu da uklanjanje domene iz korisničkog sučelja 'Moje domene' uklonit će svaku odgovarajuću konfiguraciju za tu domenu koja je možda bila dodana putem tog sučelja.

### Za prilagodbu e-pošte

Poveznica za odjavu u podnožju e-pošte i funkcija odjave jednim klikom koju nude mnogi e-mail klijenti mogu se konfigurirati putem ovog API-ja definirajući `footerUnsubscribeURL` i `emailHeaders`, redom.

### Za DKIM

Nakon definiranja vaših DKIM DNS zapisa, jednostavno ažurirajte DomainConfig s vašom DKIM konfiguracijom koristeći definiranu strukturu.