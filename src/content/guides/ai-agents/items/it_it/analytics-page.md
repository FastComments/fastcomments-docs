Analytics è la dashboard cross-agent. È raggiungibile dalla pagina AI Agents tramite la scheda **Analytics** (a livello di tenant) oppure per singolo agente tramite il pulsante **Analytics** sulla riga di ciascun agente.

### Filtro

Un menu a tendina in alto - **Tutti gli agenti** o un agente specifico. Il resto della pagina si ridimensiona di conseguenza.

### Utilizzo del budget

Quattro barre di avanzamento che mostrano la spesa del periodo corrente rispetto al limite:

- **Agente oggi** (quando filtrato su un agente specifico) - limite giornaliero dell'agente.
- **Agente questo mese** - limite mensile dell'agente.
- **Account oggi** - limite giornaliero del tenant.
- **Account questo mese** - limite mensile del tenant.

Quando un limite non è impostato, la barra mostra "(nessun limite impostato)" e indica la spesa effettiva.

### Costo giornaliero (ultimi 30 giorni)

Una tabella del costo giornaliero nella valuta del tuo tenant per l'ambito selezionato. Utile per individuare:

- **Picchi improvvisi di costo** - di solito causati da un loop fuori controllo o da un commento virale che genera a catena più trigger.
- **Deriva dei costi** - un costo giornaliero che aumenta gradualmente man mano che la tua community cresce.

### Azioni eseguite

Una ripartizione dei tipi di azione nel mese corrente - "Ha scritto un commento: 47", "Ha contrassegnato un commento come spam: 12", e così via. Utile per verificare che l'agente stia facendo ciò che ti aspettavi.

### Trigger saltati (questo mese)

Conteggi raggruppati per [motivo di scarto](#drop-reasons):

- Oltre il limite giornaliero/mensile dell'agente o giornaliero/mensile dell'account.
- Limitazione di frequenza (rate-limited).
- Concorrenza satura.

Se vedi scarti qui, il tuo agente sta raggiungendo un limite di budget o di frequenza e sta perdendo trigger che avrebbe altrimenti eseguito. Vedi [Motivi di scarto](#drop-reasons).

### Dry-run vs live (questo mese)

- **Esecuzioni abilitate** - numero di esecuzioni che hanno compiuto azioni reali questo mese.
- **Esecuzioni dry-run** - numero di esecuzioni in modalità dry-run questo mese.

Un segnale utile per la messa a punto: un agente appena creato che non è stato ancora promosso ad Enabled mostrerà solo esecuzioni dry-run. Un agente in Enabled con conteggi tutti a zero in questa sezione è inattivo - o non viene attivato, oppure viene escluso dall'ambito, oppure i suoi trigger non sono configurati correttamente.

### Agenti principali per costo mensile

Quando il filtro è **Tutti gli agenti**, la pagina elenca gli agenti classificati per costo dal mese in corso. Individuare l'agente più costoso è il primo passo per l'ottimizzazione dei costi - di solito la risposta è "restringere le sue [opzioni di contesto](#context-options)" o "abbassare il suo [limite di budget](#budgets-overview)".

### Agenti al limite o vicino al limite

Ripartizione per agente degli agenti la cui spesa è pari o vicina ai loro limiti per agente nel periodo corrente:

- **vicino al limite** - oltre una percentuale configurabile del limite.
- **oltre il limite** - effettivamente limitato, con trigger `{count} dropped` in quel periodo.

Clicca sull'agente da questa tabella per aumentare il limite, restringere l'ambito o metterlo in pausa.

### Riepilogo dell'account

Quando il filtro è **Tutti gli agenti**:

- **Trigger oggi** - conteggio.
- **Trigger questo mese** - conteggio.
- Per ciascuno: un suffisso `dropped` che indica quanti sono stati saltati.

### Valuta

Tutti i valori monetari sono mostrati nella valuta del tuo tenant.

### Cosa non fa questa pagina

- Non mostra le **ripartizioni dei costi per azione** - si trovano in [Run Detail View](#run-detail-view).
- Non mostra le **trascrizioni** o le **risposte LLM**.
- Non consente di agire sugli agenti - modifica, pausa ed eliminazione si eseguono tutti dall'elenco degli agenti / dalla pagina di modifica.