Un oggetto `DomainConfig` rappresenta la configurazione per un dominio di un tenant.

La struttura dell'oggetto `DomainConfig` è la seguente:

[inline-code-attrs-start title = 'Struttura di DomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** Un dominio, non un URL, come "fastcomments.com" o "www.example.com". È possibile includere il sottodominio se si desidera limitare a un sottodominio. Massimo 1000 caratteri. **/
    domain: string
    /** Il From-Name usato durante l'invio delle email. **/
    emailFromName?: string
    /** Il From-Email usato durante l'invio delle email. Assicurati che SPF sia configurato per consentire a mail.fastcomments.com di inviare email come il dominio usato in questo attributo. **/
    emailFromEmail?: string
    /** SOLO LETTURA. Quando l'oggetto è stato creato. **/
    createdAt: string
    /** Il logo relativo a questo dominio. Usato nelle email. Usare HTTPS. **/
    logoSrc?: string
    /** Un logo più piccolo relativo a questo dominio. Usare HTTPS. **/
    logoSrc100px?: string
    /** SOLO SSO. L'URL usato nel footer di ogni email inviata. Supporta una variabile "[userId]". **/
    footerUnsubscribeURL?: string
    /** SOLO SSO. Le intestazioni usate in ogni email inviata. Utile per esempio per impostare intestazioni relative alla disiscrizione per migliorare la consegna. L'elemento List-Unsubscribe in questo Record, se esiste, supporta una variabile "[userId]". **/
    emailHeaders?: Record<string, string>
    /** Disabilita tutti i link di disiscrizione. Non raccomandato, potrebbe danneggiare i tassi di consegna. **/
    disableUnsubscribeLinks?: boolean
    /** Configurazione DKIM. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura di Configurazione DKIM'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** Il nome di dominio nel tuo record DKIM. **/
    domainName: string
    /** Il selettore della chiave DKIM da utilizzare. **/
    keySelector: string
    /** La chiave pubblica, in formato PEM. Restituita nelle risposte GET. **/
    publicKey: string
    /** @deprecated Non più restituito nelle risposte API. Accettato in scrittura per compatibilità retroattiva. **/
    privateKey?: string
}
[inline-code-end]

### Per l'autenticazione

La configurazione del dominio viene utilizzata per determinare quali siti possono ospitare il widget FastComments per il tuo account. Questa è una forma di autenticazione di base, il che significa che aggiungere o rimuovere qualsiasi configurazione di dominio può influire sulla disponibilità della tua installazione FastComments in produzione.

Non rimuovere o aggiornare la proprietà `domain` di un `Domain Config` per un dominio attualmente in uso, a meno che non sia tua intenzione disabilitare quel dominio.

Questo ha lo stesso comportamento della rimozione di un dominio da [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Nota inoltre che rimuovere un dominio dall'interfaccia `My Domains` rimuoverà qualsiasi configurazione corrispondente per quel dominio che potrebbe essere stata aggiunta tramite tale interfaccia.

### Per la personalizzazione delle email

Il link di disiscrizione nel footer dell'email e la funzionalità di disiscrizione con un clic offerta da molti client di posta possono essere configurati tramite questa API definendo rispettivamente `footerUnsubscribeURL` e `emailHeaders`.

### Per DKIM

Dopo aver definito i record DNS DKIM, aggiorna semplicemente il DomainConfig con la tua configurazione DKIM utilizzando la struttura definita.

---