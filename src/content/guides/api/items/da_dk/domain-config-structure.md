Et `DomainConfig`-objekt repræsenterer konfiguration for et domæne for en tenant.

Strukturen for `DomainConfig`-objektet er som følger:

[inline-code-attrs-start title = 'Domain Config Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'DKIM Config Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** Your private key. Start with -----BEGIN PRIVATE KEY----- and end with -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Til Autentificering

Domænekonfiguration bruges til at bestemme, hvilke websteder der kan hoste FastComments-widget'en for din konto. Dette er en grundlæggende form
for autentificering, hvilket betyder, at tilføjelse eller fjernelse af domænekonfigurationer kan påvirke tilgængeligheden af din FastComments-installation
i produktion.

Fjern eller opdater ikke `domain`-egenskaben for en `Domain Config` for et domæne, der i øjeblikket er i brug, medmindre deaktivering af det domæne er tilsigtet.

Dette har samme adfærd som at fjerne et domæne fra [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Bemærk også, at fjernelse af et domæne fra `Mine Domæner`-brugergrænsefladen vil fjerne enhver tilsvarende konfiguration for det domæne, der måtte være blevet tilføjet via denne brugergrænseflade.

### Til E-mail-tilpasning

Afmeldingslinket i e-mail-sidefoden og et-klik-afmelding-funktionen, der tilbydes af mange e-mail-klienter, kan konfigureres via denne API ved at definere henholdsvis `footerUnsubscribeURL` og `emailHeaders`.

### Til DKIM

Efter at have defineret dine DKIM DNS-poster, skal du blot opdatere DomainConfig med din DKIM-konfiguration ved hjælp af den definerede struktur.
