La sezione **Context** del modulo di modifica controlla quante informazioni riceve l'agente a ogni esecuzione. Più contesto produce decisioni migliori ma aumenta il costo in token per esecuzione, quindi vuoi solo ciò di cui l'agente ha effettivamente bisogno.

### Cosa è sempre incluso

Anche con tutte le caselle di controllo deselezionate, il messaggio di contesto dell'agente include:

- Il **tipo di evento trigger** (es. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L'**URL della pagina e l'URL ID** (quando noto).
- Il **commento** che ha attivato l'esecuzione, se presente - ID, ID utente autore, nome visualizzato dell'autore, testo del commento, conteggi dei voti, conteggio dei flag, flag spam/approvato/revisionato, ID padre. L'email dell'autore non viene **mai** inviata al provider LLM (minimizzazione dei PII).
- Il **testo del commento precedente** per i trigger `COMMENT_EDIT` (così l'agente può confrontare prima/dopo).
- La **direzione del voto** per i trigger `COMMENT_VOTE_THRESHOLD`.
- L'**ID utente trigger** e il **badge ID** (per i trigger di badge dei moderatori).
- Il **catalogo dei badge** del tuo tenant (nome, etichetta visualizzata, descrizione) quando all'agente è consentito assegnare badge, così l'agente può sceglierne uno appropriato senza che tu debba elencare i badge nel prompt.

Tutto il testo non affidabile - corpi dei commenti, nomi degli autori, titoli delle pagine, il documento delle linee guida stesso - è **delimitato** nel messaggio di contesto con marker come `<<<COMMENT_TEXT>>> ... <<<END>>>`. Il system prompt della piattaforma istruisce il modello a non seguire mai le istruzioni all'interno di tali delimitazioni. Questa è la difesa della piattaforma contro il prompt-injection; non devi ripeterla nel tuo prompt.

### Le tre caselle di controllo

#### Includi commento padre e risposte precedenti nello stesso thread

Aggiunge:
- Il **commento padre** - ID, autore, testo.
- **Risposte sibling** - le risposte precedenti allo stesso padre nello stesso thread.

Utile per: qualsiasi agente che risponde a un commento nel contesto (saluti di benvenuto, riassuntori di thread, moderatori che leggono le risposte nelle conversazioni).

Costo: piccolo o medio. Limitato da quante risposte sibling esistono in un dato thread.

#### Includi trust factor del commentatore, età dell'account, cronologia dei ban e commenti recenti

Aggiunge il blocco **AUTHOR_HISTORY**:

- **Età dell'account in giorni** dalla registrazione.
- **Trust factor (0-100)** - il punteggio FastComments che riassume quanto l'utente è affidabile su questo sito. Vedi la pagina [Spam Detection](/guide-moderation.html#spam-detection) nella guida alla moderazione.
- **Numero di ban precedenti.**
- **Totale commenti su questo sito.**
- **Conteggio contenuti duplicati** - se l'utente ha pubblicato testo identico di recente (segnale anti-spam).
- **Segnale cross-account stesso IP** - conteggio dei commenti dallo stesso IP sotto altri account (segnale di account alternativo). L'hash IP stesso non viene mai inviato all'LLM.
- **Commenti recenti** - fino a 5 dei commenti più recenti dell'utente, ciascuno troncato a 300 caratteri, delimitati come testo non affidabile.

Utile per: qualsiasi agente di moderazione. Senza questo, il modello banna allo stesso modo nuovi account e utenti di lunga data in buona fede.

Costo: medio. I commenti recenti aggiungono più token.

#### Includi titolo della pagina, sottotitolo, descrizione e meta tag

Aggiunge il blocco **PAGE_CONTEXT** - titolo, sottotitolo, descrizione e qualsiasi meta tag che FastComments ha acquisito per la pagina.

Utile per: saluti di benvenuto e riassuntori di thread, dove sapere di cosa tratta la pagina migliora sostanzialmente la qualità dell'output.

Costo: piccolo.

### Linee guida della community

Il quarto campo, **Community guidelines**, è un blocco di policy in testo libero incluso nel messaggio di contesto user-role a ogni esecuzione, delimitato come testo non affidabile allo stesso modo in cui sono delimitati i corpi dei commenti e gli altri contenuti forniti dagli utenti. L'agente lo legge come testo di policy ma la piattaforma non lo tratta come un'istruzione di sistema. Vedi [Community Guidelines](#community-guidelines) per cosa inserirvi.

### Aggiungere contesto selettivamente

Queste caselle di controllo si applicano per agente, non globalmente. Un pattern comune:

- Saluto di benvenuto: contesto pagina **on**, contesto thread **off**, cronologia utente **off**.
- Moderatore: contesto thread **off**, cronologia utente **on**, contesto pagina **off**.
- Riassuntore di thread: contesto thread **on**, contesto pagina **on**, cronologia utente **off**.

Punta al contesto minimo di cui un agente ha bisogno per essere corretto nelle chiamate che effettua realmente - il contesto extra costa token a ogni esecuzione, anche quando l'agente non lo usa.