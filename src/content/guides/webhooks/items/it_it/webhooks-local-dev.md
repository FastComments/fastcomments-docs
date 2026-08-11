For lo sviluppo locale, usa uno strumento come [ngrok](https://ngrok.com/).

Per semplificare il mantenimento della sicurezza del sistema, lo sviluppo locale segue lo stesso processo di configurazione e protezione di altri ambienti. 

### Passo 1: Aggiungi "localhost" ai domini nel tuo account.

Aggiungi "localhost" [come dominio qui](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='The add domain form in account settings with localhost entered in the domain names field'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Passo 2: Scegli una API Key

Aggiungeremo la configurazione del webhook per il tuo dominio, quindi avremo bisogno di una chiave API. [Puoi farlo qui.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='New API secret form with the associated domain set to localhost and the key named Testing'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Sotto "Associate with domain" - seleziona il tuo dominio "localhost".

**NOTA: In alternativa, puoi usare un unico API Secret per tutte le attività di test e gli ambienti di staging. Basta aggiungere un API Secret per "All Domains" e dargli un nome come "test".**

Assicurati di avere un API Secret definito per i tuoi domini di produzione. Gli eventi per tutti gli altri domini utilizzeranno il segreto wildcard (di test).

### Passo 3: Aggiungi il tuo webhook

Mentre esegui ngrok o uno strumento simile, imposta il valore per "localhost" [qui](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Webhooks admin with the localhost domain selected and an ngrok URL filled into the comment created endpoint'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Facendo clic su `Send Test Payload`, invieremo due eventi di test per verificare che tu abbia validato la chiave API.

Una volta validato, premi `Save`.

### Passo 4: Aggiungi un commento

Ora puoi aggiungere, modificare o eliminare commenti e dovresti vedere che chiamiamo la tua macchina di sviluppo locale con gli eventi, usando la tua chiave API di test. Potrebbe esserci un ritardo fino a 30 secondi perché gli eventi raggiungano la tua macchina.

---