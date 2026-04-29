Cliccando su **Visualizza** su una riga in [Cronologia esecuzioni](#run-history) si apre la pagina dei dettagli per quella esecuzione. Qui puoi leggere il ragionamento dell'agente e giudicare le sue decisioni.

### In alto: riepilogo dell'esecuzione

- **Agent** - l'agente che ha eseguito.
- **When** - data/ora.
- **Status** - Started / Success / Error, oltre al badge **Dry Run** se applicabile.
- **Cost** - costo per esecuzione nella valuta del tuo tenant.
- **Cost per action** - costo diviso per il numero di azioni non in sospeso, utile per individuare esecuzioni insolitamente costose.

### Azioni effettuate

Un elenco, in ordine, di ogni chiamata a strumenti effettuata durante l'esecuzione. Ogni voce mostra:

- **Action label** - "Ha scritto un commento", "Ha contrassegnato un commento come spam", "Ha bannato un utente" e così via. L'etichetta corrisponde al tipo di azione nell'enum.
- **Reference ID** - l'ID del commento, utente o badge interessato, mostrato come testo monospaziato (non un collegamento).
- **Agent reasoning** - la giustificazione fornita dall'agente con la chiamata.
- **Confidence** - la fiducia auto-valutata dall'agente, mostrata come percentuale.
- **Pending approval** badge - se l'azione è messa in coda nella [approvals inbox](#approval-workflow) invece di essere eseguita.

Se l'esecuzione non ha effettuato azioni, la sezione recita: "No actions were taken during this run."

### Trascrizione LLM

Sotto le azioni, la trascrizione completa della conversazione dell'agente con l'LLM:

- **System** - il system prompt (suffisso della piattaforma + il tuo prompt iniziale + le linee guida della community).
- **User** - il messaggio di contesto che descrive il trigger.
- **Assistant** - le risposte del modello, incluse le chiamate agli strumenti.
- **Tool** - il risultato dello strumento fornito al modello (es., ciò che ha restituito `search_memory`).

I messaggi lunghi sono comprimibili; clicca **Espandi** / **Comprimi** per visualizzarli.

### Leggere le trascrizioni

La trascrizione è la pagina più importante per il tuning. Quando l'agente prende una decisione con cui non sei d'accordo, rileggila per capire:

- Cosa il modello **ha visto** (il messaggio di contesto User).
- Cosa il modello **ha deciso** (le chiamate agli strumenti dell'Assistant).
- Cosa il modello **ha considerato** (qualsiasi risultato di strumenti - es., l'agente ha effettivamente chiamato `search_memory` e ha trovato qualcosa prima di bannare).

Se il modello commette sistematicamente lo stesso tipo di errore, modifica il [prompt iniziale](#personality-prompt) - oppure usa [Refining Prompts](#refining-prompts) da un'approvazione rifiutata.

### Riferimenti alle azioni

Gli ID di riferimento sono mostrati come testo monospaziato (non collegamenti):

- Commenti: l'ID del commento.
- Utenti: l'ID dell'utente.
- Badge: l'ID del badge.

Puoi copiare l'ID per cercare il record interessato nella pagina di moderazione/amministrazione pertinente.

### Cosa manca in modalità dry-run

Le esecuzioni in dry-run mostrano le stesse azioni, giustificazioni e punteggi di confidenza. L'unica differenza è il badge **Dry Run** sulla riga di stato. Gli ID di riferimento per commenti / utenti / badge sono comunque mostrati - l'agente semplicemente non li ha modificati.

### Errori

Per le esecuzioni in stato `Error`, la pagina dei dettagli mostra il messaggio di errore sottostante. Errori comuni:

- **No LLM API key configured** - errata configurazione del tenant o della piattaforma.
- **LLM call timeout** - il provider LLM era lento o non disponibile.
- **Tool dispatch failure** - l'agente ha scelto uno strumento con argomenti errati (es., un ID commento che non esiste più).
- **Budget exhausted mid-run** - il limite dell'agente è stato raggiunto mentre l'esecuzione era in corso. L'esecuzione è stata interrotta.

Gli errori non annullano le azioni parziali - qualsiasi chiamata agli strumenti completata prima dell'errore rimane.