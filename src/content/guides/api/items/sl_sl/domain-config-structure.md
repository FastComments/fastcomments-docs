Objekt `DomainConfig` predstavlja konfiguracijo za domeno za najemnika.

Struktura objekta `DomainConfig` je naslednja:

[inline-code-attrs-start title = 'Struktura konfiguracije domene'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Domena, ne URL, na primer "fastcomments.com" ali "www.example.com". Poddomena je lahko vključena, če želite omejiti na poddomeno. Največ 1000 znakov. **/
    domain: string
    /** Ime pošiljatelja (From-Name), ki se uporablja pri pošiljanju e-pošte. **/
    emailFromName?: string
    /** E-poštni naslov pošiljatelja (From-Email), ki se uporablja pri pošiljanju e-pošte. Zagotovite, da je SPF nastavljen, tako da dovoljuje mail.fastcomments.com pošiljanje e-pošte v imenu domene, uporabljene v tem atributu. **/
    emailFromEmail?: string
    /** SAMO-ZA-BRANJE. Kdaj je bil objekt ustvarjen. **/
    createdAt: string
    /** Logotip za to domeno. Uporablja se v e-poštnih sporočilih. Uporabite HTTPS. **/
    logoSrc?: string
    /** Manjši logotip za to domeno. Uporabite HTTPS. **/
    logoSrc100px?: string
    /** SAMO SSO. URL, uporabljen v nogi vsakega poslanega e-poštnega sporočila. Podpira spremenljivko "[userId]". **/
    footerUnsubscribeURL?: string
    /** SAMO SSO. Glave, uporabljene v vsakem poslanem e-poštnem sporočilu. Uporabno na primer za nastavljanje glave povezanih z odjavo za izboljšanje dostave. Vnos List-Unsubscribe v tem zapisu, če obstaja, podpira spremenljivko "[userId]". **/
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
    /** Izbirnik (selector) DKIM ključa, ki ga je treba uporabiti. **/
    keySelector: string
    /** Javni ključ v formatu PEM. Vrnjen v GET odgovorih. **/
    publicKey: string
    /** @deprecated Ne vrne se več v API odgovorih. Sprejeto pri zapisu za združljivost z preteklimi različicami. **/
    privateKey?: string
}
[inline-code-end]

### Za avtentikacijo

Konfiguracija domene se uporablja za določitev, katere spletne strani lahko gostijo FastComments widget za vaš račun. To je osnovna oblika avtentikacije, kar pomeni, da lahko dodajanje ali odstranjevanje katere koli konfiguracije domene vpliva na razpoložljivost vaše namestitve FastComments v produkciji.

Ne odstranjujte ali posodabljajte lastnosti `domain` objekta `Domain Config` za domeno, ki je trenutno v uporabi, razen če nameravate to domeno onemogočiti.

To deluje enako kot odstranitev domene v [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Upoštevajte tudi, da bo odstranitev domene iz uporabniškega vmesnika `My Domains` odstranila tudi morebitno ustrezno konfiguracijo za to domeno, ki je bila dodana prek tega vmesnika.

### Za prilagoditev e-pošte

Povezavo za odjavo v nogi e-poštnega sporočila in funkcijo enoklikne odjave, ki jo ponuja veliko e-poštnih odjemalcev, lahko konfigurirate preko tega API z definiranjem `footerUnsubscribeURL` oziroma `emailHeaders`.

### Za DKIM

Po tem, ko definirate svoje DKIM DNS zapise, preprosto posodobite `DomainConfig` s svojo DKIM konfiguracijo z uporabo zgornje strukture.

---