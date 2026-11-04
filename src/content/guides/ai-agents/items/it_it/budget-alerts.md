Le email di avviso di budget vengono inviate quando la spesa di un agente supera una percentuale configurabile del suo limite. Vengono inviate alle persone che pagano la fattura.

### Come funzionano gli avvisi

Ogni agente ha un campo **Alert thresholds** nella form di modifica. Per impostazione predefinita è `80%` e `100%`. Puoi selezionare o deselezionare le singole soglie, e puoi aggiungere altre percentuali.

Quando la spesa dell'agente in un dato ambito (giornaliero o mensile) supera una soglia per la prima volta in quel periodo, la piattaforma invia una email per destinatario. Superare nuovamente la soglia più avanti nello stesso periodo (ad esempio, la spesa scende sotto l'80% e poi torna sopra) **non** provoca un nuovo invio.

Questo è per periodo: un nuovo reset giornaliero riavvia la logica di superamento della soglia per quel giorno.

### Avvisi a livello di tenant

Il tenant (account) ha i propri limiti giornalieri e mensili. Gli avvisi a livello di tenant si attivano a soglie fisse (`80%` e `100%`). Questi non sono configurabili per agente perché si applicano all'intero tenant.

### Destinatari

Gli avvisi di budget vengono inviati a:

- Ogni utente contrassegnato come **Super admin** sul tenant.
- Ogni utente contrassegnato come **Billing Admin** sul tenant.

Ciò include l'unione dei due ruoli - un utente con entrambi i ruoli riceve una sola email.

### Perché entrambi i ruoli

I Super admin sono tipicamente gli operatori che devono sapere quando un agente sta raggiungendo il suo limite. I Billing admin possiedono la fattura e devono sapere about picchi di costo indipendentemente dal fatto che gestiscano gli agenti quotidianamente. Per modificare effettivamente l'agente (aumentare il limite, metterlo in pausa), il destinatario necessita anche del ruolo **Customization Admin** - che protegge la pagina di modifica dell'agente.

### Rinuncia per utente

I destinatari che hanno rinunciato alle notifiche amministrative sul proprio profilo vengono saltati. Questo è lo stesso interruttore di rinuncia che controlla le altre notifiche amministrative.

Se **tutti** i destinatari hanno rinunciato, l'avviso viene registrato (livello warning) e nessuna email viene inviata.

### Contenuto dell'email

L'email contiene:

- Il **nome visualizzato dell'agente** e il nome interno.
- L'**ambito** superato (ad esempio, "agent daily budget", "agent monthly budget", "account daily budget", "account monthly budget").
- La **percentuale di soglia** superata.
- L'**utilizzo** nella valuta del tenant.
- Il **limite** nella valuta del tenant.
- Un **link di accesso firmato con un clic** che porta il destinatario direttamente a:
  - La pagina di modifica dell'agente, per gli avvisi a livello di agente.
  - La pagina dell'elenco degli AI Agents, per gli avvisi a livello di tenant.

Il link è pre-autenticato, quindi il destinatario è a un clic di distanza dall'aumentare il limite o disabilitare l'agente.

### Come si attivano le soglie

La piattaforma traccia quali soglie si sono già attivate in questo periodo, separatamente per l'agente e per il tenant. Quindi:

- Superare l'80% e poi il 100% nello stesso periodo attiva entrambe, in ordine.
- Passare direttamente da 0% a 100% in un unico grande salto attiva la soglia superata **più alta** (100%), non l'80%, quindi l'avviso più severo è quello consegnato.

### Quando smetti di ricevere avvisi

Se la spesa dell'agente non raggiunge mai la soglia successiva in questo periodo, non ricevi ulteriori email in questo periodo. Il prossimo reset giornaliero (o mensile) cancella il tracciamento.

### Disabilitazione degli avvisi

Deseleziona la soglia che non desideri. Se non vuoi alcun avviso su un agente specifico, deseleziona tutte le percentuali. Gli avvisi a livello di tenant non possono essere disabilitati per agente (sono a livello di tenant).

### Vedi anche

- [Panoramica dei Budget](#budgets-overview).
- [Motivi di scarto](#drop-reasons) - cosa succede quando il limite è completamente raggiunto.
- [Modello di costo](#cost-model) - cosa viene misurato.