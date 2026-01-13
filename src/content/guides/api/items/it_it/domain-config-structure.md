Un oggetto `DomainConfig` rappresenta la configurazione per un dominio di un tenant.

La struttura dell'oggetto `DomainConfig` è la seguente:

[inline-code-attrs-start title = 'Struttura dell\'oggetto DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Un dominio, non un URL, come "fastcomments.com" o "www.example.com". Si può includere il sottodominio se si desidera limitare a un sottodominio. Massimo 1000 caratteri. **/
    domain: string
    /** Il From-Name usato quando si inviano email. **/
    emailFromName?: string
    /** Il From-Email usato per l'invio delle email. Assicurarsi che SPF sia configurato per consentire a mail.fastcomments.com di inviare email con il dominio usato in questo attributo. **/
    emailFromEmail?: string
    /** SOLO LETTURA. Quando l'oggetto è stato creato. **/
    createdAt: string
    /** Il logo relativo a questo dominio. Usato nelle email. Usare HTTPS. **/
    logoSrc?: string
    /** Un logo più piccolo relativo a questo dominio. Usare HTTPS. **/
    logoSrc100px?: string
    /** SOLO SSO. L'URL usato nel piè di pagina di ogni email inviata. Supporta la variabile "[userId]". **/
    footerUnsubscribeURL?: string
    /** SOLO SSO. Le intestazioni usate in ogni email inviata. Utile, ad esempio, per impostare intestazioni correlate alla cancellazione dalla mailing list per migliorare la consegna. L'entry List-Unsubscribe in questo Record, se esiste, supporta la variabile "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Disabilita tutti i link di cancellazione. Non raccomandato, potrebbe compromettere i tassi di consegna. **/
    disableUnsubscribeLinks?: boolean
    /** Configurazione DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura configurazione DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Il nome di dominio nel tuo record DKIM. **/
    domainName: string
    /** Il selettore della chiave DKIM da utilizzare. **/
    keySelector: string
    /** La tua chiave privata. Inizia con -----BEGIN PRIVATE KEY----- e termina con -----END PRIVATE KEY----- **/
    privateKey: string
}
[inline-code-end]

### Per l'autenticazione

La configurazione del dominio viene usata per determinare quali siti possono ospitare il widget FastComments per il tuo account. Questa è una forma basilare di autenticazione, il che significa che aggiungere o rimuovere qualsiasi configurazione di dominio può influenzare la disponibilità della tua installazione FastComments in produzione.

Non rimuovere o aggiornare la proprietà `domain` di un `Domain Config` per un dominio attualmente in uso a meno che non sia intenzionale disabilitare quel dominio.

Questo ha lo stesso comportamento della rimozione di un dominio da [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Nota anche che rimuovere un dominio dall'interfaccia `My Domains` rimuoverà qualsiasi configurazione corrispondente per quel dominio che potrebbe essere stata aggiunta tramite questa interfaccia.

### Per la personalizzazione delle email

Il link di cancellazione nel piè di pagina dell'email, e la funzionalità di annullamento dell'iscrizione con un clic offerta da molti client di posta, possono essere configurati tramite questa API definendo rispettivamente `footerUnsubscribeURL` e `emailHeaders`.

### Per DKIM

Dopo aver definito i record DNS DKIM, aggiorna semplicemente il DomainConfig con la tua configurazione DKIM usando la struttura definita.