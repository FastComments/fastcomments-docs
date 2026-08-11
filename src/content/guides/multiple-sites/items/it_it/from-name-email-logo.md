A volte FastComments deve inviare email ai tuoi utenti, soprattutto se non utilizzi Secure SSO.

Esempi di ciò includono la verifica del loro account o dell'attività quando commentano per la prima volta. FastComments invierà loro anche notifiche per le risposte ai loro commenti.

Quando FastComments invia email ai tuoi utenti, utilizzeremo un nome e un'email predefiniti di `FastComments Robot` e `noreply@fastcomments.com`.

Utilizzeremo anche il nostro logo nel piè di pagina di queste email.

Se disponi di FastComments Flex o Pro, tutto ciò può essere personalizzato per dominio tramite la pagina "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Modulo impostazioni email per dominio con i campi Nome mittente, Email mittente e caricamento logo'; title='Personalizzazione Nome mittente, Email e Logo' app-screenshot-end]

Quando personalizzi il logo mostrato nelle email, assicurati che le dimensioni del file caricato corrispondano a quelle che desideri visualizzare nel piè di pagina dell'email.

### Quando personalizzi il `From Domain`

Se personalizzi il `From Domain`, i provider di email e i client devono sapere che FastComments è autorizzato a inviare email per tuo conto. Altrimenti, definire il `From Domain` e non seguire i passaggi seguenti probabilmente farà finire le email nello spam.

#### 1. Configurare SPF

Per consentire a FastComments di inviare email in modo sicuro come il tuo dominio, assicurati di aggiungere un record SPF che lo permetta.

Assicurati che ci siano record SPF che consentano a `mail.fastcomments.com` e `sib.fastcomments.com` di inviare email come il tuo dominio.

Ulteriori informazioni su come farlo sono disponibili qui: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Configurare DKIM

Oltre a SPF, dovresti configurare DKIM. Una volta pronta la configurazione DNS, puoi fare clic su "Show Advanced" nella pagina di configurazione dei domini per visualizzare le impostazioni DKIM per dominio.

Puoi anche [invocare l'API](/guide-api.html#domain-config-structure) per impostare la configurazione DKIM.

### Link di cancellazione iscrizione

Quando si utilizza SSO, le funzionalità di cancellazione presenti nelle email e nelle notifiche possono essere personalizzate [tramite l'API DomainConfigs](/guide-api.html#domain-config-structure).

### Offuscamento dei link email

Se la reputazione del dominio del tuo sito fa finire le email di notifica nello spam, puoi instradare i pulsanti "visualizza commento" tramite `fastcomments.com` invece di collegarli direttamente alla tua pagina. I provider di caselle di posta valutano ogni link nel corpo dell'email in base alla reputazione della destinazione, quindi quando il tuo dominio è segnalato i link diretti contribuiscono al punteggio di spam indipendentemente da quanto sia pulita la tua configurazione di invio.

Abilita questa opzione in "Show Advanced" nella pagina My Domains, nella sezione "Email Link Obfuscation". L'impostazione è per dominio.

Quando abilitato, i link nelle email di menzione, risposta, nuovo commento, pagina iscritta, commento profilo e digest vengono riscritti in token brevi che reindirizzano alla pagina originale al clic. La destinazione è legata al tuo tenant: il reindirizzamento avviene solo verso URL il cui host corrisponde a uno dei domini configurati, e i token scadono automaticamente dopo 30 giorni.

L'esperienza di click-through rimane invariata. I lettori atterrano comunque sulla tua pagina con il commento già scorrevole in vista.