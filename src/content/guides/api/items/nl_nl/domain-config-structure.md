---
Een `DomainConfig`-object vertegenwoordigt de configuratie voor een domein voor een tenant.

De structuur voor het `DomainConfig`-object is als volgt:

[inline-code-attrs-start title = 'Structuur DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Een domein, geen URL, zoals "fastcomments.com" of "www.example.com". Een subdomein kan worden opgenomen als beperking tot een subdomein gewenst is. Maximaal 1000 tekens. **/
    domain: string
    /** De From-Name die wordt gebruikt bij het verzenden van e-mails. **/
    emailFromName?: string
    /** De From-Email die wordt gebruikt bij het verzenden van e-mails. Zorg dat SPF is ingesteld om mail.fastcomments.com toe te staan e-mails te verzenden namens het domein dat in dit attribuut wordt gebruikt. **/
    emailFromEmail?: string
    /** ALLEEN-LEZEN. Wanneer het object is aangemaakt. **/
    createdAt: string
    /** Het logo dat bij dit domein hoort. Gebruikt in e-mails. Gebruik HTTPS. **/
    logoSrc?: string
    /** Een kleiner logo dat bij dit domein hoort. Gebruik HTTPS. **/
    logoSrc100px?: string
    /** ENKEL SSO. De URL die wordt gebruikt in de voettekst van elke verzonden e-mail. Ondersteunt een "[userId]"-variabele. **/
    footerUnsubscribeURL?: string
    /** ENKEL SSO. De headers die worden gebruikt in elke verzonden e-mail. Handig, bijvoorbeeld om unsubscribe-gerelateerde headers in te stellen om de aflevering te verbeteren. De List-Unsubscribe-vermelding in dit record, indien aanwezig, ondersteunt een "[userId]"-variabele. **/
    emailHeaders?: Record<string, string>
    /** Schakel alle unsubscribe-links uit. Niet aanbevolen, kan de afleveringspercentages schaden. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM-configuratie. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur DKIM-configuratie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** De domeinnaam in uw DKIM-record. **/
    domainName: string
    /** De DKIM-sleutelselector die gebruikt moet worden. **/
    keySelector: string
    /** De publieke sleutel in PEM-formaat. Teruggegeven in GET-responses. **/
    publicKey: string
    /** @deprecated Niet langer teruggegeven in API-responses. Geaccepteerd bij schrijven voor achterwaartse compatibiliteit. **/
    privateKey?: string
}
[inline-code-end]

### Voor authenticatie

Domain-configuratie wordt gebruikt om te bepalen welke sites de FastComments-widget voor uw account kunnen hosten. Dit is een basale vorm
van authenticatie, wat betekent dat het toevoegen of verwijderen van domeinconfiguraties invloed kan hebben op de beschikbaarheid van uw FastComments-installatie
in productie.

Verwijder of wijzig de `domain`-eigenschap van een `Domain Config` niet voor een domein dat momenteel in gebruik is, tenzij het de bedoeling is dat dat domein wordt uitgeschakeld.

Dit heeft hetzelfde gedrag als het verwijderen van een domein via [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Houd er ook rekening mee dat het verwijderen van een domein vanuit de `My Domains`-UI alle bijbehorende configuratie voor dat domein verwijdert die mogelijk via deze UI is toegevoegd.

### Voor e-mailaanpassing

De afmeldlink in de e-mailvoettekst en de 'one-click unsubscribe'-functie die door veel e-mailclients wordt aangeboden, kunnen via deze API worden geconfigureerd door respectievelijk `footerUnsubscribeURL` en `emailHeaders` te definiëren.

### Voor DKIM

Nadat u uw DKIM-DNS-records heeft gedefinieerd, werkt u eenvoudig de DomainConfig bij met uw DKIM-configuratie met behulp van de gedefinieerde structuur. 

---