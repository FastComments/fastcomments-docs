A volte FastComments deve inviare email ai tuoi utenti, specialmente se non stai usando SSO sicuro.

Esempi di questo includono la verifica del loro account o dell'attività quando commentano per la prima volta. FastComments
invierà anche notifiche per le risposte ai loro commenti.

Quando FastComments invia email ai tuoi utenti, useremo un From Name e un From Email predefiniti di `FastComments Robot` e `noreply@fastcomments.com`.

Useremo anche il nostro logo nel footer di queste email.

Se disponi di FastComments Flex o Pro, tutto questo può essere personalizzato per dominio tramite la pagina "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Quando personalizzi il logo mostrato nelle email, assicurati che la dimensione che stai caricando sia la stessa che vuoi mostrare nel footer dell'email.

### Quando personalizzi il `From Domain`

Se personalizzi il `From Domain`, i provider e i client di posta devono sapere che FastComments è autorizzato a inviare email per conto tuo. Altrimenti,
definire il `From Domain` senza seguire i passaggi descritti di seguito probabilmente farà sì che le email finiscano nello spam.

#### 1. Configura SPF

Per consentire a FastComments di inviare in modo sicuro email come il tuo dominio, assicurati di aggiungere un record SPF che ci autorizzi a farlo.

Assicurati che ci siano record SPF che permettano a `mail.fastcomments.com` e `sib.fastcomments.com` di inviare posta come il tuo dominio.

Alcune informazioni in più su come fare questo sono qui: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configura DKIM

Oltre a SPF, dovresti configurare DKIM. Una volta che la tua configurazione DNS è pronta, puoi cliccare su "Mostra opzioni avanzate" nella pagina di configurazione dei domini
per mostrare le impostazioni DKIM per dominio.

Puoi anche [invoke the API](/guide-api.html#domain-config-structure) per impostare la configurazione DKIM.

### Link per annullare l'iscrizione

Quando usi SSO, le funzionalità di annullamento iscrizione usate nelle email e nelle notifiche possono essere personalizzate [via the DomainConfigs API](/guide-api.html#domain-config-structure).

### Offuscamento dei link nelle email

Se la reputazione del dominio del tuo sito sta facendo finire le email di notifica nello spam, puoi instradare i pulsanti "view comment" tramite `fastcomments.com` invece di linkare direttamente alla tua pagina. I provider di posta valutano ogni link nel corpo dell'email rispetto alla reputazione della destinazione, quindi quando il tuo dominio viene segnalato i link diretti contribuiscono al punteggio di spam indipendentemente da quanto pulita sia la configurazione di invio.

Abilita questa opzione in "Mostra opzioni avanzate" nella pagina "My Domains", nella sezione "Email Link Obfuscation". L'impostazione è per dominio.

Quando abilitata, i link nelle email di mention, reply, new-comment, subscribed-page, profile-comment e digest vengono riscritti in token brevi che reindirizzano alla pagina originale al clic. La destinazione è vincolata al tuo tenant: il reindirizzamento inoltra solo verso URL il cui host corrisponde a uno dei tuoi domini configurati, e i token scadono automaticamente dopo 30 giorni.

L'esperienza dopo il clic è invariata. I lettori atterrano ancora sulla tua pagina con il commento portato in vista.

---