---
**Template ID:** `gaslight_detector`

Il Gaslight Detector monitora le modifiche ai commenti che riscrivono la storia nel mezzo di una conversazione – il tipo in cui un autore cambia il significato di un commento precedente dopo che sono state scritte le risposte, facendo apparire le risposte successive fuori contesto o errate. Quando l'agente decide che una modifica supera quel limite, ripristina il testo originale e invia un DM all'autore per spiegare.

Questo è un modello ad alto rischio perché modifica il contenuto degli utenti. Eseguilo in [dry-run](#dry-run-mode) più a lungo di quanto faresti con un modello in sola lettura, e metti `edit_comment` dietro [approval](#approval-workflow) finché non ti fidi del giudizio del modello sul tuo traffico.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) – l'agente confronta il nuovo testo con quello precedente e decide se la modifica distorce le risposte già esistenti.

Vedi [Trigger: Comment Edited](#trigger-comment-edit) per il payload completo, includendo il testo del commento precedente e il conteggio delle risposte al momento della modifica.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) – usato per ripristinare il testo originale quando la modifica è giudicata come gaslighting.
- [`warn_user`](#tool-warn-user) – emette un avviso soft che l'utente vede al suo prossimo accesso.
- [`send_dm`](#tools-overview) – il canale di spiegazione; l'utente riceve un messaggio diretto che descrive perché la sua modifica è stata ripristinata.

Non può bannare, segnalare spam, votare o pubblicare nuovi commenti – la superficie è intenzionalmente limitata.

### Recommended additions before going live

- **Gate `edit_comment` behind [approval](#approval-workflow).** Ripristinare un commento è visibile all'autore e a chiunque abbia visto la versione modificata, quindi un falso positivo è imbarazzante. Mantieni le approvazioni attive finché il dry-run non dimostra che l'agente è coerente.
- **Rafforza il prompt con ciò che conta come gaslighting sul tuo sito.** Il prompt predefinito è breve di proposito. Fornisci al modello esempi concreti – “capovolgere un'affermazione sì/no”, “cancellare un numero citato dalle risposte”, “aggiungere una frase ostile dopo che le risposte sono state pubblicate” – e non‑esempi espliciti come correzioni di errori di battitura, pulizia della formattazione o aggiunta di fonti.
- **Usa il conteggio delle risposte dal contesto del trigger.** Le modifiche a commenti con zero risposte non possono distorcere una conversazione; il prompt dovrebbe dire al modello di saltarle.
- **Seleziona "Includi il fattore di fiducia del commentatore, l'età dell'account, la cronologia dei ban e i commenti recenti"** in [Context Options](#context-options). Il modello è molto meno aggressivo quando può vedere un account di buona fede da lungo tempo.
- **Considera una breve finestra di grazia per le modifiche nel prompt.** Molte modifiche nei primi 30–60 secondi sono correzioni di errori di battitura; istruisci il modello a ignorare modifiche così rapide.

### Recommended dry-run window

Esegui per almeno due settimane di traffico reale in [dry-run](#dry-run-mode) prima di passare a Abilitato, e rivedi ogni modifica segnalata durante quella finestra. Usa [Test Runs (Replays)](#test-runs-replays) per riprodurre gli ultimi 30 giorni di modifiche contro l'agente prima di andare in produzione.

---