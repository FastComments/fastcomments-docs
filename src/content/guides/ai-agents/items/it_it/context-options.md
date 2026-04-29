La sezione **Context** nel modulo di modifica controlla quante informazioni l'agente riceve a ogni esecuzione. Più contesto produce decisioni migliori ma aumenta il costo in token per esecuzione, quindi vuoi solo ciò di cui l'agente ha effettivamente bisogno.

### Cosa è sempre incluso

Anche con tutte le caselle deselezionate, il messaggio di contesto dell'agente include:

- Il **tipo di evento trigger** (es. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L'**URL della pagina e l'ID URL** (quando noto).
- Il **commento** che ha causato l'esecuzione, se presente - ID, ID autore, nome visualizzato dell'autore, testo del commento, conteggi dei voti, conteggio dei flag, flag spam/approvato/revisionato, ID del genitore. L'email dell'autore **non** viene mai inviata al provider LLM (minimizzazione dei dati personali, PII).
- Il **testo del commento precedente** per i trigger `COMMENT_EDIT` (in modo che l'agente possa confrontare prima/dopo).
- La **direzione del voto** per i trigger `COMMENT_VOTE_THRESHOLD`.
- L'**ID utente che ha attivato il trigger** e l'**ID badge** (per i trigger di badge moderatore).
- Il tuo tenant **badge catalog** (nome, etichetta visualizzata, descrizione) quando all'agente è permesso assegnare badge, così l'agente può scegliere quello appropriato senza che tu debba elencare i badge nel prompt.

Tutto il testo non attendibile - corpi dei commenti, nomi degli autori, titoli delle pagine, il documento delle linee guida stesso - è **racchiuso** nel messaggio di contesto con marcatori come `<<<COMMENT_TEXT>>> ... <<<END>>>`. Il system prompt della piattaforma istruisce il modello a non seguire mai le istruzioni all'interno di quelle recinzioni. Questa è la difesa della piattaforma contro il prompt-injection; non è necessario ripeterla nel tuo prompt.

### Le tre caselle

#### Include parent comment and prior replies in the same thread

Aggiunge:
- Il **commento genitore** - ID, autore, testo.
- **Risposte fratelli** - le risposte precedenti allo stesso genitore nello stesso thread.

Utile per: qualsiasi agente che risponde a un commento in contesto (bot di benvenuto, riassuntori di thread, moderatori che leggono le risposte nelle conversazioni).

Costo: da piccolo a medio. Limitato dal numero di fratelli presenti in un dato thread.

#### Include commenter's trust factor, account age, ban history, and recent comments

Aggiunge il blocco **AUTHOR_HISTORY**:

- **Età dell'account in giorni** dal momento della registrazione.
- **Trust factor (0-100)** - il punteggio FastComments che riepiloga quanto è affidabile l'utente su questo sito. Vedi la pagina [Rilevamento dello spam](/guide-moderation.html#spam-detection) nella guida alla moderazione.
- **Numero di ban precedenti.**
- **Totale commenti su questo sito.**
- **Conteggio di contenuti duplicati** - se l'utente ha pubblicato testo identico di recente (segnale anti-spam).
- **Segnale cross-account stesso IP** - conteggio dei commenti dallo stesso IP sotto altri account (segnale di alt-account). L'hash dell'IP non viene mai inviato all'LLM.
- **Commenti recenti** - fino a 5 degli ultimi commenti dell'utente, ciascuno troncato a 300 caratteri, racchiusi come testo non attendibile.

Utile per: qualsiasi agente di moderazione. Senza questo, il modello tende a bannare account nuovi e utenti di lunga data in buona fede che presentano la stessa postura.

Costo: medio. I commenti recenti aggiungono il maggior numero di token.

#### Include page title, subtitle, description, and meta tags

Aggiunge il blocco **PAGE_CONTEXT** - titolo, sottotitolo, descrizione e qualsiasi meta tag che FastComments ha catturato per la pagina.

Utile per: bot di benvenuto e riassuntori di thread, dove sapere di cosa parla la pagina migliora sostanzialmente la qualità dell'output.

Costo: basso.

### Linee guida della community

Il quarto campo, **Community guidelines**, è un blocco di policy in testo libero incluso nel messaggio di contesto con ruolo utente a ogni esecuzione, racchiuso come testo non attendibile allo stesso modo dei corpi dei commenti e di altri contenuti forniti dall'utente. L'agente lo legge come testo di policy ma la piattaforma non lo considera un'istruzione di sistema. Vedi le [Linee guida della community](#community-guidelines) per cosa inserire al suo interno.

### Aggiungere contesto selettivamente

Queste caselle si applicano per agente, non globalmente. Uno schema comune:

- Bot di benvenuto: page context **attivo**, thread context **disattivo**, user history **disattivo**.
- Moderatore: thread context **disattivo**, user history **attivo**, page context **disattivo**.
- Riassuntore di thread: thread context **attivo**, page context **attivo**, user history **disattivo**.

Scegli il minimo contesto di cui un agente ha bisogno per essere corretto nelle chiamate che effettivamente esegue - il contesto extra costa token a ogni esecuzione, anche quando l'agente non lo usa.