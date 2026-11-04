**ID del template:** `gaslight_detector`

Il Gaslight Detector controlla le modifiche ai commenti che riscrivono la storia nel mezzo di una conversazione - il tipo in cui un autore cambia il significato di un commento precedente dopo che le risposte sono state scritte, lasciando le risposte successive fuori contesto o sbagliate. Quando l'agente decide che una modifica supera quella linea, ripristina il testo originale e invia un DM all'autore per spiegare.

Questo è un template a rischio più elevato perché modifica i contenuti degli utenti. Eseguilo in [dry-run](#dry-run-mode) più a lungo di quanto faresti con un template di sola lettura, e vincola `edit_comment` dietro [approvazione](#approval-workflow) finché non ti fidi del giudizio del modello sul tuo traffico.

### Trigger

- **Commento modificato** (`COMMENT_EDIT`) - l'agente confronta il nuovo testo con quello precedente e decide se la modifica altera le risposte già esistenti.

Vedi [Trigger: Comment Edited](#trigger-comment-edit) per il payload completo, incluso il testo precedente del commento e il numero di risposte al momento della modifica.

### Strumenti consentiti

- [`edit_comment`](#tool-edit-comment) - usato per ripristinare il testo originale quando la modifica viene giudicata come gaslighting.
- [`warn_user`](#tool-warn-user) - emette un avviso soft che l'utente vede alla sua prossima visita.
- [`send_dm`](#tools-overview) - il canale di spiegazione; l'utente riceve un messaggio diretto che descrive il motivo del ripristino della sua modifica.

Non può bannare, segnare come spam, votare o pubblicare nuovi commenti - la superficie è volutamente ristretta.

### Aggiunte consigliate prima di andare in produzione

- **Vincola `edit_comment` dietro [approvazione](#approval-workflow).** Il ripristino di un commento è visibile all'autore e a chiunque abbia visto la versione modificata, quindi un falso positivo è imbarazzante. Mantieni le approvazioni attive finché il dry-run non mostra che l'agente è coerente.
- **Affina il prompt con ciò che conta come gaslighting sul tuo sito.** Il prompt predefinito è volutamente breve. Dai al modello esempi concreti - "invertire un'affermazione sì/no", "eliminare un numero citato dalle risposte", "aggiungere una frase ostile dopo che le risposte sono state pubblicate" - ed espliciti controesempi come correzioni di refusi, pulizia della formattazione o aggiunta di fonti.
- **Usa il numero di risposte dal contesto del trigger.** Le modifiche a commenti senza risposte non possono alterare una conversazione; il prompt dovrebbe dire al modello di ignorarle.
- **Spunta "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). Il modello è molto meno aggressivo quando può vedere un account di lunga data e di buona fede.
- **Considera una breve finestra di grazia per le modifiche nel prompt.** Molte modifiche nei primi 30-60 secondi sono correzioni di refusi; istruisci il modello a ignorare modifiche così rapide.

### Finestra di dry-run consigliata

Esegui per almeno due settimane di traffico reale in [dry-run](#dry-run-mode) prima di passare a Enabled, e rivedi ogni modifica segnalata durante quella finestra. Usa [Test Runs (Replays)](#test-runs-replays) per riprodurre gli ultimi 30 giorni di modifiche contro l'agente prima di andare in produzione.