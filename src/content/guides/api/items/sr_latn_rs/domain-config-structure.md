Objekat `DomainConfig` predstavlja konfiguraciju za domen za tenant.

Struktura objekta `DomainConfig` je sledeća:

[inline-code-attrs-start title = 'Struktura objekta DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string
    /** The From-Name used when sending emails. **/
    emailFromName?: string
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string
    /** A smaller logo related to this domain. Use HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura DKIM konfiguracije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** The public key, in PEM format. Returned in GET responses. **/
    publicKey: string
    /** @deprecated No longer returned in API responses. Accepted on write for backwards compatibility. **/
    privateKey?: string
}
[inline-code-end]

### Za autentifikaciju

Konfiguracija domena se koristi da odredi koje sajtove mogu da hostuju FastComments widget za vaš nalog. Ovo je osnovni oblik
autentifikacije, što znači da dodavanje ili uklanjanje bilo kojih konfiguracija domena može uticati na dostupnost vaše FastComments instalacije
u produkciji.

Ne uklanjajte niti ne ažurirajte `domain` svojstvo `Domain Config` za domen koji se trenutno koristi osim ako nije namera da se taj domen onemogući.

Ovo ima isto ponašanje kao uklanjanje domena sa [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Takođe imajte na umu da uklanjanje domena iz `My Domains` UI ukloniće bilo koju odgovarajuću konfiguraciju za taj domen koja je možda dodata putem ovog UI.

### Za prilagođavanje e-pošte

Link za odjavu u podnožju e-pošte, i funkcija jednim klikom za odjavu koju nude mnogi klijenti e-pošte, mogu se konfigurisati putem ovog API-ja definisanjem `footerUnsubscribeURL` i `emailHeaders`, redom.

### Za DKIM

Nakon što definišete DKIM DNS zapise, jednostavno ažurirajte DomainConfig sa vašom DKIM konfiguracijom koristeći definisanu strukturu. 

---