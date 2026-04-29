Un **trigger** è un evento che risveglia un agente. Ogni agente può avere uno o più trigger definiti.

### Elenco completo

| Trigger | Quando si attiva |
|---|---|
| [Commento aggiunto](#trigger-comment-add) | Viene pubblicato un nuovo commento. |
| [Commento modificato](#trigger-comment-edit) | Un commento viene modificato. Il testo precedente è incluso nel contesto dell'agente. |
| [Commento eliminato](#trigger-comment-delete) | Un commento viene eliminato. |
| [Commento fissato](#trigger-comment-pin) | Un commento viene fissato (da chiunque, inclusi un moderatore o un altro agente). |
| [Commento rimosso dal pin](#trigger-comment-unpin) | Un commento viene rimosso dal pin. |
| [Commento bloccato](#trigger-comment-lock) | Un commento viene bloccato (non sono consentite ulteriori risposte). |
| [Commento sbloccato](#trigger-comment-unlock) | Un commento viene sbloccato. |
| [Commento supera la soglia di voti](#trigger-comment-vote-threshold) | I voti netti di un commento raggiungono la soglia configurata. |
| [Commento supera la soglia di segnalazioni](#trigger-comment-flag-threshold) | Il conteggio delle segnalazioni di un commento raggiunge esattamente la soglia configurata. |
| [Utente pubblica il primo commento](#trigger-new-user-first-comment) | Un utente pubblica il suo primo commento su questo sito. |
| [Commento marcato come spam automaticamente](#trigger-comment-auto-spammed) | Un commento viene auto-segnalato come spam dal motore anti-spam. |
| [Moderatore esamina il commento](#trigger-moderator-reviewed) | Un moderatore segna un commento come esaminato. |
| [Moderatore approva il commento](#trigger-moderator-approved) | Un moderatore approva un commento. |
| [Moderatore segnala come spam](#trigger-moderator-spammed) | Un moderatore segnala un commento come spam. |
| [Moderatore assegna un badge](#trigger-moderator-awarded-badge) | Un moderatore assegna un badge a un utente. |

### Più trigger per agente

Un agente può iscriversi a qualsiasi combinazione di trigger - il [Modello Moderatore](#template-moderator) si iscrive sia a `COMMENT_ADD` che a `COMMENT_FLAG_THRESHOLD`, per esempio. Ogni evento attiva l'agente una sola volta con il contesto di quell'evento.

### Cosa impedisce l'attivazione di un agente

Un evento trigger a cui ci si è iscritti **non** attiva l'agente se si verifica una qualsiasi delle condizioni seguenti:

- Lo [stato](#status-states) dell'agente è **Disabilitato**.
- L'[ambito URL o locale](#scope-url-locale) dell'agente non corrisponde al commento che ha scatenato l'evento.
- Il [budget giornaliero, mensile o di rate-limit](#budgets-overview) dell'agente è esaurito - il trigger viene registrato come **scartato** con una motivazione. Vedi [Motivi di scarto](#drop-reasons).
- La concorrenza per questo agente è saturata (limite per agente).
- Il tenant dell'agente ha fatturazione non valida.
- L'azione che ha scatenato il trigger è stata eseguita da un bot o da un altro agente (prevenzione dei loop).
- Il trigger riguardava un commento che è già stato processato da questo agente entro la finestra di deduplicazione.

Quando un trigger a cui ci si è iscritti si attiva con successo, la [Cronologia esecuzioni](#run-history) dell'agente mostra una riga con stato **Avviato** che transita a **Successo** o **Errore** al completamento dell'esecuzione.

### Soglie di voti e segnalazioni

Due trigger - **Commento supera la soglia di voti** e **Commento supera la soglia di segnalazioni** - richiedono una soglia numerica nel modulo di modifica. Il trigger si attiva nel momento in cui il conteggio supera il valore configurato (in particolare, il trigger per la soglia di segnalazioni si attiva quando `flagCount === flagThreshold`, quindi scegliere 1 significa "attivarsi alla prima segnalazione", e scegliere 5 significa "attivarsi quando arriva la quinta segnalazione").

### Trigger differiti

Qualsiasi trigger può essere differito in modo che l'agente venga eseguito in un secondo momento, per esempio dopo che voti/segnalazioni/risposte hanno avuto il tempo di stabilizzarsi. Vedi [Trigger differiti](#trigger-deferred-delay).

### Prevenzione dei loop

Per prevenire loop infiniti i commenti **scritto da un agente** portano un `botId`. I trigger di nuovo commento ignorano i commenti con un `botId`.

Effetto netto: gli agenti possono agire in risposta ad azioni *umane* nel tuo tenant, ma le azioni generate dagli agenti non attivano mai i trigger degli agenti. Questo vale per tutti i tipi di trigger.

### REPLAY: il trigger interno

Esiste anche un tipo di trigger interno `REPLAY` usato dalla funzionalità [Esecuzioni di test (Replay)](#test-runs-replays). Non è possibile selezionarlo nel modulo di modifica - esiste affinché le esecuzioni di replay siano etichettate in modo distinto nella cronologia esecuzioni ed escluse dalle visualizzazioni delle esecuzioni live.