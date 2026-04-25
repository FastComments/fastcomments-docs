Una sottoinsieme di azioni di moderazione può essere eseguito direttamente dal thread dei commenti, senza dover andare alla pagina di Moderazione dei Commenti.

Quando sei autenticato, fai clic sul pulsante di modifica in alto a destra di un commento. Come moderatore dovresti avere le seguenti opzioni:

- **Fissa** quel commento
- **Elimina** quel commento
- **Elimina** quel commento + **Banna l'utente** (Permanente o Shadow, più dettagli dopo)
- **Modifica** quel commento
- **Blocca** o **Sblocca** quel commento (più dettagli sotto)
- Segna quel commento **Approvato** (mostralo) o **Non approvato** (nascondilo)
- Segna quel commento come **Spam** o **Non Spam**

### Blocco di un commento

Bloccare un singolo commento impedisce qualsiasi nuova risposta ad esso e impedisce anche che il commento stesso venga modificato o eliminato finché non viene sbloccato. Questo si applica a tutti, inclusi amministratori e moderatori. Se hai bisogno di modificare o rimuovere un commento bloccato, sbloccalo prima, effettua la modifica e, se desideri, bloccane nuovamente.

Un'icona a forma di lucchetto appare nell'angolo in alto a destra di un commento bloccato in modo che i lettori possano vedere a colpo d'occhio che il thread è chiuso. Le voci di menu Modifica ed Elimina sono nascoste per i commenti bloccati sia nel widget dei commenti sia nell'API pubblica (`PATCH` e `DELETE` restituiscono `code: 'locked'` se chiamati su un commento bloccato).

Due eccezioni intenzionali eludono il blocco, perché altrimenti lascerebbero dati orfani: quando un utente elimina il proprio intero account (i suoi commenti vengono rimossi indipendentemente dallo stato di blocco), e quando un moderatore banna un utente con l'opzione "delete all comments from this user" (la pulizia rimuove anche i blocchi).

### Chiusura dei thread dei commenti

I moderatori e gli amministratori possono bloccare, o chiudere, i thread dei commenti selezionando `Close Thread` nel menu a tre puntini nella parte superiore dell'area dei commenti, se sono autenticati. Possono selezionare `Re-Open Thread` in seguito, in qualsiasi momento, per riaprire i commenti.

La chiusura di un thread di commenti impedisce nuovi commenti, ma permette comunque di votare e agli utenti di eliminare i propri commenti, se lo desiderano.

La chiusura e la riapertura dei thread di commenti influiscono immediatamente su tutti gli utenti che stanno visualizzando il thread.

Puoi anche contrassegnare un thread come sola lettura rimuovendo anche le opzioni di voto ed eliminazione, creando una regola di personalizzazione specifica per quella pagina.

### Aggiornamenti in tempo reale

Tutte queste azioni aggiorneranno immediatamente i thread dei commenti degli altri utenti senza che debbano ricaricare la pagina. Tuttavia, le azioni del moderatore come nascondere un commento o segnarlo come spam, non rimuovono il commento dallo schermo **del moderatore** in modo che, se necessario, possano annullare rapidamente l'azione. Per indicare che il commento è nascosto, verrà evidenziato rispetto agli altri commenti (il colore dell'evidenziazione dipende dal motivo della rimozione).

Per esempio, dati gli utenti `A (commenter)`, `B (Moderator 1)`, e `C (Moderator 2)`.

...e il seguente scenario:

1. `User B (Moderator 1)` nasconde un commento.
2. Per `User A (commenter)` quel commento viene immediatamente nascosto.
3. Per `User C (Moderator 2)` quel commento viene immediatamente nascosto.
4. Per l'utente che ha effettuato la modifica, `User B (Moderator 1)`, il commento rimane sul loro schermo, ma è evidenziato come rimosso. Hanno l'opzione di annullare la loro azione, nel qual caso gli altri utenti vedranno di nuovo l'aggiornamento in tempo reale.