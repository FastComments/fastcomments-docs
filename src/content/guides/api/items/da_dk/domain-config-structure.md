Et `DomainConfig`-objekt repræsenterer konfiguration for et domæne for en lejer.

Strukturen for `DomainConfig`-objektet er som følger:

[inline-code-attrs-start title = 'Struktur for domænekonfiguration'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Et domæne, ikke en URL, som "fastcomments.com" eller "www.example.com". Subdomæne kan inkluderes, hvis det er ønsket at begrænse til et subdomæne. Maks 1000 tegn. **/
    domain: string
    /** Det From-Name, der bruges ved afsendelse af e-mails. **/
    emailFromName?: string
    /** From-Email der bruges ved afsendelse af e-mails. Sørg for, at SPF er opsat for at tillade mail.fastcomments.com at sende e-mails som det domæne, der bruges i denne attribut. **/
    emailFromEmail?: string
    /** READONLY. Hvornår objektet blev oprettet. **/
    createdAt: string
    /** Logo knyttet til dette domæne. Bruges i e-mails. Brug HTTPS. **/
    logoSrc?: string
    /** Et mindre logo knyttet til dette domæne. Brug HTTPS. **/
    logoSrc100px?: string
    /** KUN SSO. URL'en, der bruges i footeren i alle sendte e-mails. Understøtter en "[userId]"-variabel. **/
    footerUnsubscribeURL?: string
    /** KUN SSO. Headers, der bruges i alle sendte e-mails. Nyttigt, for eksempel til at sætte unsubscribe-relaterede headers for at forbedre levering. List-Unsubscribe-indgangen i denne Record, hvis den findes, understøtter en "[userId]"-variabel. **/
    emailHeaders?: Record<string, string>
    /** Deaktiver alle afmeldingslinks. Anbefales ikke, kan skade leveringsraten. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM-konfiguration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for DKIM-konfiguration'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Domænenavnet i din DKIM-post. **/
    domainName: string
    /** DKIM-nøglevælgeren der skal bruges. **/
    keySelector: string
    /** Den offentlige nøgle i PEM-format. Returneres i GET-responser. **/
    publicKey: string
    /** @deprecated Returneres ikke længere i API-respons. Accepteres ved skrivning for tilbagekompatibilitet. **/
    privateKey?: string
}
[inline-code-end]

### Til autentificering

Domænekonfiguration bruges til at bestemme, hvilke sider der kan hoste FastComments-widgetten for din konto. Dette er en grundlæggende form
for autentificering, hvilket betyder, at tilføjelse eller fjernelse af domænekonfigurationer kan påvirke tilgængeligheden af din FastComments-installation
i produktion.

Fjern eller opdater ikke `domain`-egenskaben for en `Domain Config` for et domæne, der i øjeblikket er i brug, medmindre formålet er at deaktivere det domæne.

Dette har samme adfærd som at fjerne et domæne fra [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Bemærk også, at fjernelse af et domæne fra `My Domains`-brugerfladen vil fjerne enhver tilsvarende konfiguration for det domæne, som måtte være blevet tilføjet via denne brugerflade.

### Til tilpasning af e-mails

Afmeldingslinket i e-mail-footeren og one-click-unsubscribe-funktionen, som tilbydes af mange e-mail-klienter, kan konfigureres via dette API ved hhv. at definere `footerUnsubscribeURL` og `emailHeaders`.

### Til DKIM

Efter du har defineret dine DKIM DNS-poster, opdater blot DomainConfig med din DKIM-konfiguration ved hjælp af den definerede struktur. 

---