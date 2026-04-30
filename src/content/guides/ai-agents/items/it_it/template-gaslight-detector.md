---
**ID del template:** `gaslight_detector`

Il Gaslight Detector osserva le modifiche ai commenti che riscrivono la storia nel mezzo di una conversazione - quel tipo in cui un autore cambia il significato di un commento precedente dopo che sono state scritte delle risposte, lasciando le risposte successive fuori contesto o errate. Quando l'agente decide che una modifica supera quella soglia, ripristina il testo originale e invia un DM all'autore per spiegare.

Questo è un template a rischio più elevato perché modifica i contenuti degli utenti. Eseguilo in [dry-run](#dry-run-mode) più a lungo rispetto a quanto faresti con un template in sola lettura, e metti `edit_comment` dietro [approval](#approval-workflow) finché non ti fiderai del giudizio del modello sul tuo traffico.

### Attivatori

- **Comment edited** (`COMMENT_EDIT`) - l'agente confronta il nuovo testo con quello precedente e decide se la modifica distorce le risposte già esistenti.

Vedi [Trigger: Comment Edited](#trigger-comment-edit) per il payload completo, incluso il testo precedente del commento e il conteggio delle risposte al momento della modifica.

### Strumenti consentiti

- [`edit_comment`](#tool-edit-comment) - usato per ripristinare il testo originale quando la modifica è giudicata gaslighting.
- [`warn_user`](#tool-warn-user) - invia un avviso soft che l'utente vedrà alla sua prossima visita.
- [`send_dm`](#tools-overview) - il canale di spiegazione; l'utente riceve un messaggio diretto che descrive perché la sua modifica è stata annullata.

Non può bannare, contrassegnare come spam, votare o pubblicare nuovi commenti - l'ambito è intenzionalmente ristretto.

### Raccomandazioni da applicare prima di andare in produzione

- **Metti `edit_comment` dietro [approval](#approval-workflow).** Ripristinare un commento è visibile all'autore e a chiunque abbia visto la versione modificata, quindi un falso positivo è imbarazzante. Mantieni le approvazioni attive finché la modalità [dry-run](#dry-run-mode) non dimostra che l'agente è coerente.
- **Rafforza il prompt con cosa conta come gaslighting sul tuo sito.** Il prompt predefinito è volutamente breve. Fornisci al modello esempi concreti - "capovolgere un'affermazione sì/no", "cancellare un numero che le risposte citano", "aggiungere una frase ostile dopo che sono state postate risposte" - e controesempi espliciti come correzioni di refusi, pulizia del formato o aggiunta di fonti.
- **Usa il conteggio delle risposte dal contesto del trigger.** Le modifiche ai commenti con zero risposte non possono deformare una conversazione; il prompt dovrebbe dire al modello di ignorare quei casi.
- **Seleziona "Includi il fattore di fiducia del commentatore, l'età dell'account, la cronologia dei ban e i commenti recenti"** in [Context Options](#context-options). Il modello è molto meno aggressivo quando può vedere un account di lunga data che agisce in buona fede.
- **Considera una breve finestra di grazia per le modifiche nel prompt.** Molte modifiche nei primi 30-60 secondi sono correzioni di refusi; istruisci il modello a ignorare le modifiche fatte così rapidamente.

### Finestra di dry-run raccomandata

Esegui per almeno due settimane di traffico reale in [dry-run](#dry-run-mode) prima di passare a Enabled, e rivedi ogni modifica segnalata durante quel periodo. Usa [Test Runs (Replays)](#test-runs-replays) per riprodurre gli ultimi 30 giorni di modifiche contro l'agente prima di andare live.

---