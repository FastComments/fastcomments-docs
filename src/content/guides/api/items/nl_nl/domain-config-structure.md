Een `DomainConfig`-object vertegenwoordigt de configuratie voor een domein van een tenant.

De structuur voor het `DomainConfig`-object is als volgt:

[inline-code-attrs-start title = 'Structuur van Domain Config'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Een domein, geen URL, zoals "fastcomments.com" of "www.example.com". Subdomein kan worden opgenomen als beperking tot een subdomein gewenst is. Maximaal 1000 tekens. **/
    domain: string
    /** De afzendernaam die gebruikt wordt bij het verzenden van e-mails. **/
    emailFromName?: string
    /** Het From-Email-adres dat wordt gebruikt bij het verzenden van e-mails. Zorg dat SPF is ingesteld om mail.fastcomments.com toe te staan e-mails te verzenden namens het domein dat in dit attribuut wordt gebruikt. **/
    emailFromEmail?: string
    /** ALLEEN-LEZEN. Wanneer het object is aangemaakt. **/
    createdAt: string
    /** Het logo dat bij dit domein hoort. Gebruikt in e-mails. Gebruik HTTPS. **/
    logoSrc?: string
    /** Een kleiner logo dat bij dit domein hoort. Gebruik HTTPS. **/
    logoSrc100px?: string
    /** ALLEEN SSO. De URL die wordt gebruikt in de voettekst van elke verzonden e-mail. Ondersteunt een "[userId]"-variabele. **/
    footerUnsubscribeURL?: string
    /** ALLEEN SSO. De headers die in elke verzonden e-mail worden gebruikt. Handig bijvoorbeeld voor het instellen van unsubscribe-gerelateerde headers om de bezorging te verbeteren. De List-Unsubscribe-vermelding in dit Record, indien aanwezig, ondersteunt een "[userId]"-variabele. **/
    emailHeaders?: Record<string, string>
    /** Schakel alle afmeldlinks uit. Niet aanbevolen, kan de bezorgingspercentages schaden. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM-configuratie. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van DKIM-configuratie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** De domeinnaam in uw DKIM-record. **/
    domainName: string
    /** De DKIM-sleutelselector die gebruikt moet worden. **/
    keySelector: string
    /** Uw privésleutel. Begint met -----BEGIN PRIVATE KEY----- en eindigt met -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Voor authenticatie

Domeinconfiguratie wordt gebruikt om te bepalen welke sites de FastComments-widget voor uw account kunnen hosten. Dit is een basale vorm van authenticatie, wat betekent dat het toevoegen of verwijderen van domeinconfiguraties de beschikbaarheid van uw FastComments-installatie in productie kan beïnvloeden.

Verwijder of werk de `domain`-eigenschap van een `Domain Config` voor een domein dat momenteel in gebruik is niet bij, tenzij het de bedoeling is dat dat domein wordt uitgeschakeld.

Dit heeft hetzelfde gedrag als het verwijderen van een domein via [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Let ook op dat het verwijderen van een domein uit de `My Domains`-interface alle overeenkomstige configuratie voor dat domein verwijdert die mogelijk via deze interface is toegevoegd.

### Voor e-mailaanpassing

De afmeldlink in de e-mailvoettekst en de één-klik-afmelden-functie die door veel e-mailclients wordt aangeboden, kunnen via deze API worden geconfigureerd door respectievelijk `footerUnsubscribeURL` en `emailHeaders` te definiëren.

### Voor DKIM

Nadat u uw DKIM DNS-records hebt gedefinieerd, werkt u eenvoudig de DomainConfig bij met uw DKIM-configuratie met behulp van de opgegeven structuur.

---