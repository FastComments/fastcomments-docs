A volte FastComments deve inviare email ai tuoi utenti, specialmente se non stai usando Secure SSO.

Esempi di questo includono la verifica del loro account o dell'attività quando commentano per la prima volta. FastComments
invierà anche notifiche per le risposte ai loro commenti.

Quando FastComments invia email ai tuoi utenti, useremo un From Name e un indirizzo Email predefiniti di `FastComments Robot` e `noreply@fastcomments.com`.

Utilizzeremo inoltre il nostro logo nel footer di queste email.

Se hai FastComments Flex o Pro, tutto questo può essere personalizzato su base per-dominio tramite la pagina "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Quando personalizzi il logo mostrato nelle email, assicurati che la dimensione che stai caricando sia la stessa che vuoi mostrare nel footer dell'email.

### When Customizing The `From Domain`

Se personalizzi il `From Domain`, i provider e i client di posta devono sapere che FastComments è autorizzato a inviare email per tuo conto. Altrimenti,
definire il `From Domain` e non seguire i passaggi descritti di seguito probabilmente farà sì che le email finiscano nella posta indesiderata.

#### 1. Impostare SPF

Per permettere a FastComments di inviare in modo sicuro email come il tuo dominio, assicurati di aggiungere un record SPF che ci autorizzi a farlo.

Assicurati che ci siano record SPF che consentano a `mail.fastcomments.com` e `sib.fastcomments.com` di inviare posta come il tuo dominio.

Ulteriori informazioni su come farlo sono qui: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Impostare DKIM

Oltre a SPF, dovresti configurare DKIM. Una volta che la tua configurazione DNS è pronta, puoi cliccare su "Show Advanced" nella pagina di configurazione del dominio
per mostrare le impostazioni DKIM per dominio.

Puoi anche [invocare l'API](/guide-api.html#domain-config-structure) per impostare la configurazione DKIM.

### Unsubscribe Links

Quando utilizzi SSO, le funzionalità di disiscrizione usate nelle email e nelle notifiche possono essere personalizzate [tramite l'API DomainConfigs](/guide-api.html#domain-config-structure).