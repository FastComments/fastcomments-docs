Una **esecuzione di test** (chiamata anche **replay**) esegue l'agente su una finestra di commenti storici **senza effettuare azioni reali**. È il modo più veloce per visualizzare in anteprima il comportamento dell'agente prima di andare in produzione.

Accessibile dalla pagina dell'elenco degli agenti tramite il pulsante **Esecuzione di test** sulla riga di ciascun agente.

### Cosa fa

La piattaforma:

1. Seleziona un campione di commenti storici corrispondenti all'ambito dell'agente, nella finestra che scegli.
2. Per ogni commento, esegue l'agente end-to-end come se il commento fosse appena stato pubblicato - stesso contesto, stessa chiamata LLM, stessa selezione di strumenti, stesse giustificazioni e punteggi di confidenza.
3. Registra ogni esecuzione come dry-run, etichettata in modo che resti raggruppata con il replay da cui proviene ed esclusa dalle viste delle esecuzioni live.
4. **Confronta** il verdetto dell'agente con ciò che è effettivamente successo al commento - è stato successivamente approvato, contrassegnato come spam, eliminato, bloccato dal motore antispam, ecc.

Il risultato è un diff per commento: "L'agente in replay avrebbe contrassegnato questo commento come spam, ma il commento è attualmente approvato e pulito."

### Configurazione

La pagina della esecuzione di test dispone di un unico campo:

- **Giorni di commenti storici da valutare** - un campo numerico `days` compreso tra 1 e 90. I commenti più vecchi non sono idonei.

La dimensione del campione e il limite massimo non sono esposti nell'interfaccia utente - entrambi sono valori predefiniti lato server applicati per piano. La pagina mostra campi informativi:

- **Commenti corrispondenti nella finestra** - quanti commenti verrebbero considerati.
- **Fino a N commenti da questa finestra saranno processati** - la dimensione effettiva del campione data dal limite lato server.
- **Costo stimato** - nella valuta del tuo tenant.

### Limite di frequenza

Ogni utente è limitato a **10 esecuzioni di test ogni 24 ore** (rate-limited via key `replay-create:${requestedBy}`). Il pulsante mostra un tooltip quando hai raggiunto il limite ("Hai raggiunto 10 esecuzioni di test nelle ultime 24 ore.").

### Concorrenza

Solo un replay può essere attivo per agente alla volta. Avviare un secondo replay mentre uno è in corso ti reindirizza a quello in corso.

### Lettura dei risultati

Quando il replay termina, la pagina dei risultati mostra schede:

- **Divergenze** (attiva di default) - l'esito dell'agente in replay differisce dalla realtà. (Il più interessante - "l'agente avrebbe contrassegnato questo commento come spam, ma il commento è stato approvato ed è a posto".)
- **Corrispondenze** - l'esito dell'agente in replay corrisponde a ciò che è realmente successo. (Rassicurante - l'agente concorda con la realtà.)
- **Nessuna azione** - l'agente in replay ha deciso di non fare nulla. (A volte la risposta corretta; a volte l'agente ha mancato qualcosa.)
- **Tutti** - ogni risultato indipendentemente dalla classificazione.

Per ogni commento in qualsiasi scheda:

- **Esito precedente** - la classificazione di ciò che è realmente accaduto: **POSITIVO**, **NEGATIVO** o **INDETERMINATO**, con **Prove** ("Commento contrassegnato come eliminato il {date}", "Engine: bayes", e così via).
- **L'agente in replay avrebbe** - l'azione scelta dall'agente.
- **Perché** - la giustificazione.
- **Confidenza** - visualizzata come percentuale.

### Perché i replay forzano il dry-run

Un replay su un commento eliminato quattro mesi fa non dovrebbe eliminarlo retroattivamente - è già eliminato. Un replay su un commento che ora l'agente vuole approvare non dovrebbe cambiare lo stato attuale del commento. Il replay è uno strumento di anteprima. Forzare il dry-run è ciò che rende sicuro eseguire un replay su qualsiasi finestra di storia.

### Riproducibilità

I replay congelano la configurazione dell'agente al momento dell'avvio del replay. Le modifiche successive all'agente non cambiano i risultati del replay - la pagina dei risultati rimane stabile come registrazione di ciò che *quella* versione dell'agente avrebbe fatto.

### Quando i budget interrompono un replay

I replay sono soggetti a:

- Il proprio **limite massimo** (impostato nel modulo del replay).
- I **limiti di budget** giornalieri e mensili dell'agente.
- I **limiti di budget** giornalieri e mensili del tenant.

Il primo che viene raggiunto interrompe il replay con un codice di errore specifico. Eventuali risultati per commento prodotti prima dell'interruzione sono conservati in [Cronologia esecuzioni](#run-history).

### Come vengono eseguiti i replay

I replay vengono eseguiti in background, non in modo sincrono. Dopo aver cliccato "Avvia esecuzione di test", il replay viene messo in coda e un worker lo prende in carico. Un replay lungo può durare diversi minuti. La pagina dei risultati esegue polling e mostra i progressi (conteggio processati, spesa finora) mentre procede.

Se un worker muore a metà replay, la piattaforma rimette automaticamente il replay in coda così da riprendere al passaggio successivo. Un breve malfunzionamento non abbandona mai un replay.

### Cosa non fa un replay

- **Non rispetta i [ritardi di trigger](#trigger-deferred-delay).** I replay vengono eseguiti immediatamente, non 30 minuti dopo.
- **Non scrive nella memoria.** Gli agenti in replay non salvano note di memoria, anche se la loro logica lo farebbe normalmente.
- **Non invia webhook.** I trigger prodotti dal replay non generano eventi webhook `trigger.succeeded` / `trigger.failed`.
- **Non esclude i commenti già riprodotti.** Eseguire un secondo replay sulla stessa finestra copre gli stessi commenti.

### Vedi anche

- [Affinare i prompt](#refining-prompts) - il flusso di lavoro di modifica iterativa che si abbina bene ai replay.
- [Modalità Dry-Run](#dry-run-mode) - la stessa idea, applicata al traffico in tempo reale.

---