Objekt `DomainConfig` predstavlja konfiguraciju za domenu za tenant.

Struktura objekta `DomainConfig` je sljedeća:

[inline-code-attrs-start title = 'Struktura konfiguracije domene'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domena, ne URL, npr. "fastcomments.com" ili "www.example.com". Poddomena se može uključiti ako želite ograničiti na poddomenu. Maksimalno 1000 znakova. **/
    domain: string
    /** From-Name koji se koristi pri slanju e-pošte. **/
    emailFromName?: string
    /** From-Email koji se koristi pri slanju e-pošte. Osigurajte da je SPF postavljen kako bi mail.fastcomments.com mogao slati e-poštu u ime domene navedene u ovom atributu. **/
    emailFromEmail?: string
    /** SAMO ZA ČITANJE. Kada je objekt kreiran. **/
    createdAt: string
    /** Logo vezan za ovu domenu. Koristi se u e-porukama. Koristite HTTPS. **/
    logoSrc?: string
    /** Manji logo vezan za ovu domenu. Koristite HTTPS. **/
    logoSrc100px?: string
    /** SAMO SSO. URL koji se koristi u podnožju svake poslane e-pošte. Podržava varijablu "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO SSO. Zaglavlja koja se koriste u svakoj poslanoj e-pošti. Korisno, na primjer, za postavljanje zaglavlja povezanih s odjavljivanjem kako bi se poboljšala isporuka. Unos List-Unsubscribe u ovom zapisu, ako postoji, podržava varijablu "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Onemogući sve poveznice za odjavu. Ne preporučuje se, može negativno utjecati na stopu isporuke. **/
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
    /** DKIM selektor ključa koji će se koristiti. **/
    keySelector: string
    /** Vaš privatni ključ. Počinje s -----BEGIN PRIVATE KEY----- i završava s -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Za autentifikaciju

Konfiguracija domene koristi se za određivanje koje web-lokacije mogu biti domaćini FastComments widgeta za vaš račun. To je osnovni oblik autentifikacije, što znači da dodavanje ili uklanjanje bilo koje konfiguracije domene može utjecati na dostupnost vaše FastComments instalacije u produkciji.

Nemojte uklanjati ili ažurirati svojstvo `domain` u `Domain Config` za domenu koja je trenutno u upotrebi, osim ako nije namjera onemogućiti tu domenu.

Ovo ima isto ponašanje kao uklanjanje domene iz [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Također imajte na umu da će uklanjanje domene iz korisničkog sučelja `My Domains` ukloniti svaku odgovarajuću konfiguraciju za tu domenu koja je možda bila dodana putem tog sučelja.

### Za prilagodbu e-pošte

Poveznica za odjavu u podnožju e-pošte i funkcija jednoklik odjave koju nude mnogi e-mail klijenti mogu se konfigurirati putem ovog API-ja definirajući `footerUnsubscribeURL` i `emailHeaders`, redom.

### Za DKIM

Nakon što definirate svoje DKIM DNS zapise, jednostavno ažurirajte `DomainConfig` s vašom DKIM konfiguracijom koristeći zadanu strukturu. 

---