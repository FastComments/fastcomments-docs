Per lo sviluppo locale, usa uno strumento come [ngrok](https://ngrok.com/).

Per semplificare il mantenimento della sicurezza del sistema, lo sviluppo locale segue lo stesso processo di configurazione e messa in sicurezza di altri ambienti. 

### Passo 1: Aggiungi "localhost" ai domini nel tuo account.

Aggiungi "localhost" [come dominio qui](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Passo 2: Scegli un'API Key

Aggiungeremo la configurazione del webhook per il tuo dominio, quindi avremo bisogno di un'API key. [Puoi farlo qui.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Nella sezione "Associate with domain" - seleziona il tuo dominio "localhost".

**NOTA: In alternativa, puoi usare un unico API Secret per tutte le attività di test e gli ambienti di staging. Aggiungi semplicemente un API Secret per "All Domains", e assegnagli un nome come "test".**

Assicurati di avere un API Secret definito per i tuoi domini di produzione. Gli eventi per tutti gli altri domini utilizzeranno il secret wildcard (di testing).

### Passo 3: Aggiungi il tuo Webhook

Mentre esegui ngrok o uno strumento simile, imposta il valore per "localhost" [qui](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, invieremo due eventi di test per verificare che la tua API key sia valida.

Una volta convalidata, premi `Save`.

### Passo 4: Aggiungi un commento

Ora puoi aggiungere, modificare o eliminare commenti e dovresti vederci chiamare la tua macchina di sviluppo locale con gli eventi, usando la tua API key di test. Potrebbe esserci un ritardo fino a 30 secondi
perché gli eventi raggiungano la tua macchina.