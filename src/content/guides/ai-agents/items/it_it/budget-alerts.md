---
Le email di avviso sul budget vengono inviate quando la spesa di un agente supera una percentuale configurabile del suo limite. Vengono inoltrate alle persone che si occupano della fattura.

### Come funzionano gli avvisi

Each agent has an **Alert thresholds** field on the edit form. By default it is `80%` and `100%`. You can tick or untick individual thresholds, and you can add other percentages.

Quando la spesa dell'agente in uno specifico ambito (giornaliero o mensile) supera una soglia per la prima volta in quel periodo, la piattaforma invia una email per ogni destinatario. Superare di nuovo la soglia nello stesso periodo (ad es., la spesa è scesa sotto l'80% e poi è risalita) non provoca un nuovo invio.

Questo è per periodo: un nuovo reset giornaliero riavvia la logica di rilevamento delle soglie per quel giorno.

### Avvisi a livello di tenant

Il tenant (account) ha i propri limiti giornalieri e mensili. Gli avvisi a livello di tenant vengono attivati a soglie fisse (`80%` e `100%`). Non sono configurabili per singolo agente perché si applicano all'intero tenant.

### Destinatari

- Ogni utente contrassegnato come **Super admin** nel tenant.
- Ogni utente contrassegnato come **Billing Admin** nel tenant.

Ciò include l'unione dei due ruoli - un utente con entrambi i ruoli riceve una sola email.

### Perché entrambi i ruoli

I **Super admin** sono tipicamente gli operatori che devono sapere se un agente sta raggiungendo il suo limite. I **Billing Admin** sono i responsabili della fattura e devono essere informati sui picchi di costo indipendentemente dal fatto che gestiscano gli agenti quotidianamente. Per modificare effettivamente l'agente (aumentare il limite, metterlo in pausa), il destinatario ha inoltre bisogno del ruolo **Customization Admin** - che controlla l'accesso alla pagina di modifica dell'agente.

### Opt-out per utente

I destinatari che hanno scelto di non ricevere notifiche amministrative nel loro profilo vengono saltati. Questa è la stessa impostazione di opt-out che controlla altre notifiche amministrative.

Se **tutti** i destinatari hanno fatto opt-out, l'avviso viene registrato (livello warning) e non viene inviata nessuna email.

### Contenuto della email

L'email contiene:

- Il **nome visualizzato dell'agente** e il nome interno.
- L'**ambito** che è stato superato (es., "budget giornaliero dell'agente", "budget mensile dell'agente", "budget giornaliero dell'account", "budget mensile dell'account").
- La **percentuale di soglia** superata.
- **Utilizzo** nella valuta del tenant.
- **Limite** nella valuta del tenant.
- Un **link di accesso firmato con un clic** che porta il destinatario direttamente a:
  - La pagina di modifica dell'agente, per avvisi a livello agente.
  - La pagina Lista AI Agents, per avvisi a livello tenant.

Il link è pre-autenticato, quindi il destinatario è a un clic dall'aumentare il limite o dal disabilitare l'agente.

### Come si attivano le soglie

La piattaforma tiene traccia delle soglie già attivate in questo periodo, separatamente per l'agente e per il tenant. Quindi:

- Superare l'80% e poi il 100% nello stesso periodo attiva entrambe, in ordine.
- Passare direttamente da 0% al 100% con un salto unico attiva la soglia più alta superata (**100%**), non l'80%, quindi viene inviato l'avviso più grave.

### Quando smetti di ricevere avvisi

Se la spesa dell'agente non raggiunge mai la soglia successiva in questo periodo, non riceverai altre email durante lo stesso periodo. Il prossimo reset giornaliero (o mensile) azzera il tracciamento.

### Disattivare gli avvisi

Deseleziona la soglia che non desideri. Se non vuoi alcun avviso per un agente specifico, deseleziona tutte le percentuali. Gli avvisi a livello tenant non possono essere disattivati per singolo agente (sono a livello tenant).

### Vedi anche

- [Panoramica dei budget](#budgets-overview).
- [Motivi di scarto](#drop-reasons) - cosa succede quando il limite viene raggiunto completamente.
- [Modello dei costi](#cost-model) - cosa viene misurato.

---