Analytics è la dashboard trasversale agli agenti. Accessibile dalla pagina Agenti AI tramite la scheda **Analisi** (a livello di tenant) o per singolo agente tramite il pulsante **Analisi** nella riga di ciascun agente.

### Filter

Un menu a discesa in alto - **Tutti gli agenti** o un agente specifico. Il resto della pagina si adatta di conseguenza.

### Budget usage

Quattro barre di avanzamento che mostrano la spesa del periodo corrente rispetto al limite:

- **Agent today** (quando filtrato per un agente specifico) - limite giornaliero per agente.
- **Agent this month** - limite mensile per agente.
- **Account today** - limite giornaliero del tenant.
- **Account this month** - limite mensile del tenant.

Quando un limite non è impostato, la barra riporta "(nessun limite impostato)" e mostra la spesa lorda.

### Daily cost (last 30 days)

Una tabella dei costi per giorno nella valuta del tuo tenant per l'ambito selezionato. Utile per individuare:

- **Picchi improvvisi di costo** - solitamente dovuti a un loop fuori controllo o a un commento virale che scatena molte azioni.
- **Deriva dei costi** - aumento graduale del costo giornaliero man mano che la tua community cresce.

### Actions taken

Un dettaglio dei tipi di azione nel mese corrente - "Ha scritto un commento: 47", "Ha segnato un commento come spam: 12" e così via. Utile per verificare che l'agente stia facendo ciò che ti aspettavi.

### Triggers skipped (this month)

Conteggi raggruppati per [motivo di scarto](#drop-reasons):

- Oltre limite giornaliero agente / limite mensile agente / limite giornaliero account / limite mensile account.
- Limitati dal rate limit.
- Concorrenza saturata.

Se vedi scarti qui, il tuo agente sta raggiungendo un limite di budget o di velocità e sta perdendo trigger su cui altrimenti avrebbe eseguito azioni. Vedi [Drop Reasons](#drop-reasons).

### Dry-run vs live (this month)

- **Esecuzioni abilitate** - numero di esecuzioni che hanno effettuato azioni reali questo mese.
- **Esecuzioni simulate** - numero di esecuzioni in modalità dry-run questo mese.

Un segnale utile per il tuning: un agente appena creato che non è ancora stato promosso ad Abilitato mostrerà solo esecuzioni simulate. Un agente Abilitato con conteggi tutti a zero in questa sezione è inattivo - o non viene attivato, o è escluso dall'ambito, o i suoi trigger non sono configurati correttamente.

### Top agents by monthly cost

Quando il filtro è **Tutti gli agenti**, la pagina elenca gli agenti ordinati per costo da inizio mese. Individuare il tuo agente più costoso è il primo passo per l'ottimizzazione dei costi - di solito la risposta è "stringere le sue [opzioni di contesto](#context-options)" o "abbassare il suo [budget cap](#budgets-overview)".

### Agents at or near their cap

Ripartizione per agente di quelli il cui consumo è al limite o vicino al limite per agente nel periodo corrente:

- **near cap** - oltre una percentuale configurabile del limite.
- **over cap** - effettivamente limitato, con `{count} dropped` trigger in quel periodo.

Clicca sull'agente da questa tabella per aumentare il limite, restringere l'ambito o metterlo in pausa.

### Account summary

Quando il filtro è **Tutti gli agenti**:

- **Triggers today** - conteggio.
- **Triggers this month** - conteggio.
- Per ciascuno: un suffisso `dropped` che mostra quanti sono stati saltati.

### Currency

Tutti i valori monetari sono mostrati nella valuta del tuo tenant.

### What this page does not do

- Non mostra **ripartizioni dei costi per singola azione** - quelle si trovano nella [Visualizzazione dettagli esecuzione](#run-detail-view).
- Non mostra **trascrizioni** o **risposte LLM**.
- Non permette di intraprendere azioni sugli agenti - la modifica, la sospensione e la cancellazione vengono eseguite dalla lista agenti / dalla pagina di modifica.