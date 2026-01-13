---
A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = 'Struktura konfiguracije domene'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domena, ne URL, na primer "fastcomments.com" ali "www.example.com". Lahko vključuje poddomeno, če želite omejiti na poddomeno. Največ 1000 znakov. **/
    domain: string
    /** Ime pošiljatelja (From-Name), uporabljeno pri pošiljanju e-pošte. **/
    emailFromName?: string
    /** E-pošta pošiljatelja (From-Email), uporabljena pri pošiljanju e-pošte. Poskrbite, da je SPF nastavljen tako, da dovoljuje mail.fastcomments.com pošiljanje e-pošte v imenu domene, uporabljene v tem atributu. **/
    emailFromEmail?: string
    /** SAMO ZA BRANJE. Kdaj je bil objekt ustvarjen. **/
    createdAt: string
    /** Logotip povezan s to domeno. Uporablja se v e-poštnih sporočilih. Uporabite HTTPS. **/
    logoSrc?: string
    /** Manjši logotip povezan s to domeno. Uporabite HTTPS. **/
    logoSrc100px?: string
    /** SAMO SSO. URL uporabljen v nogi vsakega poslanega e-poštnega sporočila. Podpira spremenljivko "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO SSO. Glave, uporabljene v vsakem poslanem e-poštnem sporočilu. Na primer uporabno za nastavitev glav, povezanih z odjavo, za izboljšanje dostave. Vnos List-Unsubscribe v tem zapisu, če obstaja, podpira spremenljivko "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Onemogoči vse povezave za odjavo. Ni priporočljivo, lahko poslabša stopnje dostave. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM konfiguracija. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura DKIM konfiguracije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Ime domene v vašem DKIM zapisu. **/
    domainName: string
    /** Selektor DKIM ključa, ki ga je treba uporabiti. **/
    keySelector: string
    /** Vaš zasebni ključ. Začne se z -----BEGIN PRIVATE KEY----- in konča z -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Za avtentikacijo

Konfiguracija domen se uporablja za določanje, kateri spletni mesti lahko gostijo vtičnik FastComments za vaš račun. To je osnovna oblika avtentikacije, kar pomeni, da lahko dodajanje ali odstranjevanje katere koli konfiguracije domene vpliva na razpoložljivost vaše namestitve FastComments v produkcijskem okolju.

Ne odstranjujte ali ne spreminjajte lastnosti `domain` v `Domain Config` za domeno, ki se trenutno uporablja, razen če nameravate to domeno onemogočiti.

To ima enako vedenje kot odstranitev domene iz [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Upoštevajte tudi, da bo odstranitev domene iz uporabniškega vmesnika `My Domains` odstranila tudi ustrezno konfiguracijo za to domeno, ki je bila morda dodana prek tega vmesnika.

### Za prilagajanje e-pošte

Povezavo za odjavo v nogi e-poštnega sporočila in funkcijo enoklikne odjave, ki jo ponuja veliko e-poštnih odjemalcev, je mogoče konfigurirati prek tega API-ja z nastavitvijo `footerUnsubscribeURL` oziroma `emailHeaders`.

### Za DKIM

Po opredelitvi vaših DKIM DNS zapisov preprosto posodobite `DomainConfig` z vašo DKIM konfiguracijo, pri čemer uporabite zgoraj določeno strukturo. 

---