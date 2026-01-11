---
Per lo sviluppo locale, usa uno strumento come [ngrok](https://ngrok.com/).

Per semplificare il mantenimento della sicurezza del sistema, lo sviluppo locale segue lo stesso processo di configurazione e protezione degli altri ambienti. 

### Passo 1: Aggiungi "localhost" ai domini del tuo account.

Aggiungi "localhost" [come dominio qui](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Passo 2: Scegli una chiave API

Aggiungeremo una configurazione webhook per il tuo dominio, quindi avremo bisogno di una chiave API. [Puoi farlo qui.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOTA: In alternativa, puoi usare un unico API Secret per tutte le attività di test e gli ambienti di staging. Aggiungi semplicemente un API Secret per "All Domains", e dagli un nome come "test".**

Assicurati di avere un API Secret definito per i tuoi domini di produzione. Gli eventi per tutti gli altri domini useranno il secret wildcard (di test).

### Passo 3: Aggiungi il tuo Webhook

Mentre esegui ngrok o uno strumento simile, imposta il valore per "localhost" [qui](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Cliccando `Send Test Payload`, invieremo due eventi di prova per verificare che la chiave API venga validata correttamente.

Una volta validata, premi `Save`.

### Passo 4: Aggiungi un commento

Ora puoi aggiungere, modificare o eliminare commenti e dovresti vedere che chiamiamo la tua macchina di sviluppo locale con gli eventi, utilizzando la tua chiave API di test. Potrebbero esserci fino a 30 secondi di ritardo
perché gli eventi raggiungano la tua macchina.

---