Ein `DomainConfig`-Objekt stellt die Konfiguration für eine Domain eines Mandanten dar.

Die Struktur des `DomainConfig`-Objekts ist wie folgt:

[inline-code-attrs-start title = 'Struktur der Domain-Konfiguration'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Eine Domain, keine URL, z. B. "fastcomments.com" oder "www.example.com". Ein Subdomain-Anteil kann eingeschlossen werden, wenn die Einschränkung auf eine Subdomain gewünscht ist. Maximal 1000 Zeichen. **/
    domain: string
    /** Der Absendername, der beim Versenden von E-Mails verwendet wird. **/
    emailFromName?: string
    /** Die Absender-E-Mail, die beim Versenden von E-Mails verwendet wird. Stellen Sie sicher, dass SPF eingerichtet ist, um mail.fastcomments.com das Versenden von E-Mails im Namen der in diesem Attribut verwendeten Domain zu erlauben. **/
    emailFromEmail?: string
    /** SCHREIBGESCHÜTZT. Wann das Objekt erstellt wurde. **/
    createdAt: string
    /** Das zur Domain gehörige Logo. Wird in E-Mails verwendet. Verwenden Sie HTTPS. **/
    logoSrc?: string
    /** Ein kleineres zur Domain gehörendes Logo. Verwenden Sie HTTPS. **/
    logoSrc100px?: string
    /** NUR SSO. Die URL, die in der Fußzeile jeder gesendeten E-Mail verwendet wird. Unterstützt eine "[userId]"-Variable. **/
    footerUnsubscribeURL?: string
    /** NUR SSO. Die Header, die in jeder gesendeten E-Mail verwendet werden. Nützlich beispielsweise zum Setzen von unsubscribe-bezogenen Headern zur Verbesserung der Zustellbarkeit. Der List-Unsubscribe-Eintrag in diesem Record, falls vorhanden, unterstützt eine "[userId]"-Variable. **/
    emailHeaders?: Record<string, string>
    /** Alle Abmeldelinks deaktivieren. Nicht empfohlen, kann die Zustellraten beeinträchtigen. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM-Konfiguration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der DKIM-Konfiguration'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Der Domain-Name in Ihrem DKIM-Eintrag. **/
    domainName: string
    /** Der DKIM-Key-Selector, der verwendet werden soll. **/
    keySelector: string
    /** Der öffentliche Schlüssel im PEM-Format. Wird in GET-Antworten zurückgegeben. **/
    publicKey: string
    /** @deprecated Wird nicht mehr in API-Antworten zurückgegeben. Beim Schreiben aus Gründen der Abwärtskompatibilität akzeptiert. **/
    privateKey?: string
}
[inline-code-end]

### Für die Authentifizierung

Die Domain-Konfiguration wird verwendet, um zu bestimmen, welche Websites das FastComments-Widget für Ihr Konto hosten können. Dies ist eine grundlegende Form
der Authentifizierung, was bedeutet, dass das Hinzufügen oder Entfernen von Domain-Konfigurationen die Verfügbarkeit Ihrer FastComments-Installation
in der Produktion beeinträchtigen kann.

Entfernen oder aktualisieren Sie nicht die `domain`-Eigenschaft einer `Domain Config` für eine Domain, die derzeit verwendet wird, es sei denn, das Deaktivieren dieser Domain ist beabsichtigt.

Dies hat das gleiche Verhalten wie das Entfernen einer Domain über [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Beachten Sie auch, dass das Entfernen einer Domain aus der `My Domains`-Benutzeroberfläche alle entsprechenden Konfigurationen für diese Domain entfernt, die möglicherweise über diese Benutzeroberfläche hinzugefügt wurden.

### Für E-Mail-Anpassungen

Der Abmeldelink in der E-Mail-Fußzeile und die One-Click-Abmeldefunktion, die viele E-Mail-Clients anbieten, können über diese API konfiguriert werden, indem jeweils `footerUnsubscribeURL` und `emailHeaders` definiert werden.

### Für DKIM

Nachdem Sie Ihre DKIM-DNS-Einträge definiert haben, aktualisieren Sie einfach das DomainConfig mit Ihrer DKIM-Konfiguration unter Verwendung der definierten Struktur.

---