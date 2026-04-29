La sezione **Context** nel modulo di modifica controlla quante informazioni l'agente riceve ad ogni esecuzione. Più contesto produce decisioni migliori ma aumenta il costo in token per esecuzione, quindi vuoi solo ciò di cui l'agente ha effettivamente bisogno.

### Cosa è sempre incluso

Anche con tutte le caselle deselezionate, il messaggio di contesto dell'agente include:

- Il **tipo di evento trigger** (es. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- L'**URL della pagina e l'ID URL** (quando noti).
- Il **commento** che ha attivato l'esecuzione, se presente - ID, user ID dell'autore, nome visualizzato dell'autore, testo del commento, conteggi dei voti, conteggio flag, flag spam/approvato/revisionato, parent ID. L'email dell'autore **non** viene **mai** inviata al provider LLM (minimizzazione delle PII).
- Il **testo del commento precedente** per i trigger `COMMENT_EDIT` (così l'agente può confrontare prima/dopo).
- La **direzione del voto** per i trigger `COMMENT_VOTE_THRESHOLD`.
- L'**user ID** che ha innescato l'evento e l'**ID del badge** (per i trigger dei badge moderatore).

Tutto il testo non attendibile - corpi dei commenti, nomi degli autori, titoli delle pagine, il documento delle linee guida stesso - è **racchiuso** nel messaggio di contesto con marcatori come `<<<COMMENT_TEXT>>> ... <<<END>>>`. Il system prompt della piattaforma istruisce il modello a non seguire mai le istruzioni all'interno di quelle recinzioni. Questa è la difesa della piattaforma contro il prompt injection; non è necessario ripeterla nel tuo prompt.

### Le tre caselle

#### Includi il commento genitore e le risposte precedenti nello stesso thread

Aggiunge:
- Il **commento genitore** - ID, autore, testo.
- **Risposte correlate** - le risposte precedenti allo stesso commento genitore nello stesso thread.

Utile per: qualsiasi agente che risponde a un commento nel suo contesto (addetti al benvenuto, riepilogatori di thread, moderatori che leggono le risposte nelle conversazioni).

Costo: piccolo o medio. Limitato dal numero di risposte correlate presenti in un dato thread.

#### Includi il fattore di fiducia del commentatore, l'età dell'account, la cronologia dei ban e i commenti recenti

Aggiunge il blocco **AUTHOR_HISTORY**:

- **Età dell'account in giorni** dalla registrazione.
- **Fattore di fiducia (0-100)** - il punteggio FastComments che sintetizza quanto l'utente è considerato affidabile su questo sito. Vedi la pagina [Spam Detection](/guide-moderation.html#spam-detection) nella guida alla moderazione.
- **Conteggio precedenti ban.**
- **Totale commenti su questo sito.**
- **Conteggio di contenuti duplicati** - se l'utente ha pubblicato testo identico di recente (segnale anti-spam).
- **Segnale cross-account da stessa IP** - conteggio di commenti dallo stesso IP sotto altri account (segnale di alt-account). L'hash dell'IP non viene mai inviato all'LLM.
- **Commenti recenti** - fino a 5 dei commenti più recenti dell'utente, ciascuno troncato a 300 caratteri, racchiusi come testo non attendibile.

Utile per: qualsiasi agente di moderazione. Senza questo, il modello banna account nuovi e utenti di lunga data in buona fede con lo stesso comportamento.

Costo: medio. I commenti recenti aggiungono il maggior numero di token.

#### Includi titolo della pagina, sottotitolo, descrizione e meta tag

Aggiunge il blocco **PAGE_CONTEXT** - titolo, sottotitolo, descrizione e qualsiasi meta tag che FastComments abbia acquisito per la pagina.

Utile per: addetti al benvenuto e riepilogatori di thread, dove conoscere l'argomento della pagina migliora sostanzialmente la qualità dell'output.

Costo: piccolo.

### Linee guida della community

Il quarto campo, **Community guidelines**, è un blocco di policy in testo libero incluso nel messaggio di contesto con ruolo utente ad ogni esecuzione, racchiuso come testo non attendibile allo stesso modo dei corpi dei commenti e di altri contenuti forniti dagli utenti. L'agente lo legge come testo di policy ma la piattaforma non lo tratta come un'istruzione di sistema. Vedi [Community Guidelines](#community-guidelines) per cosa inserire.

### Aggiungere contesto selettivamente

Queste caselle si applicano per agente, non globalmente. Un pattern comune:

- Addetto al benvenuto: contesto pagina **attivo**, contesto thread **disattivo**, cronologia utente **disattiva**.
- Moderatore: contesto thread **disattivo**, cronologia utente **attiva**, contesto pagina **disattivo**.
- Riepilogatore di thread: contesto thread **attivo**, contesto pagina **attivo**, cronologia utente **disattiva**.

Usa il minimo contesto di cui un agente ha bisogno per essere corretto nelle chiamate che effettua realmente - il contesto extra costa token a ogni esecuzione, anche quando l'agente non lo usa.