Ein `DomainConfig`-Objekt repräsentiert die Konfiguration für eine Domain eines Tenants.

Die Struktur des `DomainConfig`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Domain-Konfiguration Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'DKIM-Konfiguration Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Für Authentifizierung

Die Domain-Konfiguration wird verwendet, um zu bestimmen, welche Websites das FastComments-Widget für Ihr Konto hosten können. Dies ist eine grundlegende Form
der Authentifizierung, was bedeutet, dass das Hinzufügen oder Entfernen von Domain-Konfigurationen die Verfügbarkeit Ihrer FastComments-Installation
in der Produktion beeinflussen kann.

Entfernen oder aktualisieren Sie die `domain`-Eigenschaft einer `Domain Config` für eine Domain, die derzeit verwendet wird, nicht, es sei denn, das Deaktivieren dieser Domain ist beabsichtigt.

Dies hat das gleiche Verhalten wie das Entfernen einer Domain aus [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Beachten Sie auch, dass das Entfernen einer Domain aus der `Meine Domains`-Benutzeroberfläche alle entsprechenden Konfigurationen für diese Domain entfernt, die möglicherweise über diese Benutzeroberfläche hinzugefügt wurden.

### Für E-Mail-Anpassung

Der Abmeldelink in der E-Mail-Fußzeile und die Ein-Klick-Abmeldefunktion, die von vielen E-Mail-Clients angeboten wird, können über diese API konfiguriert werden, indem `footerUnsubscribeURL` bzw. `emailHeaders` definiert werden.

### Für DKIM

Nachdem Sie Ihre DKIM-DNS-Einträge definiert haben, aktualisieren Sie einfach die DomainConfig mit Ihrer DKIM-Konfiguration unter Verwendung der definierten Struktur.
