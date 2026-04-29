Questo è il percorso di cinque minuti da "abbiamo Agenti AI" a "un agente sta rispondendo al traffico live, controllato da approvazioni." Se vuoi la versione estesa, ogni passaggio collega alla pagina che lo tratta in dettaglio.

### 1. Apri la pagina Agenti AI

Vai a [Agenti AI](https://fastcomments.com/auth/my-account/ai-agents) nel tuo account. La prima volta che accedi vedrai o:

- Uno stato vuoto con un pulsante **Sfoglia modelli** e **Inizia da zero** (hai agenti disponibili da creare), oppure
- Una pagina di upsell se il tuo piano non include agenti - vedi [Piani e idoneità](#plans-and-eligibility).

### 2. Scegli un modello iniziale

Fai clic su **Sfoglia modelli**. Scegli uno dei seguenti:

- [Moderatore](#template-moderator) - esamina commenti segnalati o nuovi, avverte i neofiti, provvede al ban solo dopo un avvertimento.
- [Accoglienza](#template-welcome-greeter) - risponde a chi commenta per la prima volta.
- [Fissa commenti importanti](#template-top-comment-pinner) - fissa i commenti sostanziali una volta superata una soglia di voti.
- [Riepilogo discussione](#template-thread-summarizer) - pubblica un riassunto neutrale nelle discussioni lunghe.

Ogni modello apre un modulo di modifica pre-compilato con **Stato: Esecuzione di prova** già selezionato.

### 3. Rivedi e salva

Nel modulo di modifica, compila almeno:

- **Nome interno.** Un identificatore breve utilizzato nelle dashboard di amministrazione.
- **Nome visualizzato.** Ciò che appare pubblicamente quando l'agente pubblica un commento.
- **Prompt iniziale.** Modifica il prompt del modello per adattarlo alla tua voce e alle tue regole specifiche.
- **Approvazioni.** Seleziona le azioni che dovrebbero richiedere revisione umana prima di avere effetto. Raccomandiamo almeno `ban_user` per qualsiasi agente di moderazione. Vedi [Flusso di approvazione](#approval-workflow).

Fai clic su **Salva agente**.

### 4. Osservalo in modalità Esecuzione di prova

L'agente è ora attivo in **Esecuzione di prova**. Riceverà i suoi trigger, chiamerà il modello e registrerà le azioni nella pagina [Cronologia esecuzioni](#run-history) - con il badge **Esecuzione di prova** su ogni riga - ma non esegue azioni reali. Visita alcuni dei dettagli dell'esecuzione (vedi [Vista dettaglio esecuzione](#run-detail-view)) e guarda:

- Le azioni selezionate dall'agente.
- La giustificazione e il livello di confidenza per ciascuna azione.
- La trascrizione completa del LLM.

Se l'agente prende decisioni con cui non sei d'accordo, modifica il prompt iniziale o seleziona più approvazioni.

### 5. Esegui un test sui commenti precedenti

Dalla pagina dell'elenco agenti, fai clic su **Esegui test** sulla riga dell'agente. Il modulo ha un unico campo numerico **Giorni** (1–90). La dimensione del campione e il limite massimo sui commenti valutati sono mostrati a solo scopo informativo - sono calcolati lato server, non impostabili dall'utente. La riproduzione viene eseguita sui commenti storici senza intraprendere azioni reali e riporta cosa l'agente **avrebbe** fatto rispetto a quanto è effettivamente accaduto (il commento è stato poi approvato, contrassegnato come spam, cancellato e così via). Vedi [Esecuzioni di test (Replay)](#test-runs-replays).

### 6. Passa a Abilitato

Quando sei soddisfatto dell'output dell'esecuzione di prova e del replay, modifica l'agente e cambia **Stato** in **Abilitato**. Da questo momento in poi, le azioni reali vengono eseguite. La pagina Cronologia esecuzioni ora mostra esecuzioni live senza il badge di esecuzione di prova, e qualsiasi azione che hai contrassegnato per approvazione appare nella [inbox approvazioni](#approval-workflow).

### Cosa fare dopo

- Imposta [Budget](#budgets-overview) e [Avvisi di budget](#budget-alerts).
- Configura i [Webhooks](#webhooks-overview) se desideri che sistemi esterni reagiscano agli eventi dell'agente.
- Aggiungi le [Linee guida della community](#community-guidelines) per mantenere le decisioni dell'agente allineate alla tua politica scritta.