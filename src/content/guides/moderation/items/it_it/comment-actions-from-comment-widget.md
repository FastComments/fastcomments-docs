Una sotto-insieme di azioni di moderazione può essere eseguita direttamente dal thread dei commenti, senza dover andare alla pagina Moderazione Commenti.

Quando hai effettuato l'accesso, fai clic sul pulsante di modifica in alto a destra di un commento. Dovresti avere le seguenti opzioni come moderatore:

- **Mettere in evidenza** quel commento
- **Eliminare** quel commento
- **Eliminare** quel commento + **Bannare l'utente** (Permanente o Shadow, più dettagli più avanti)
- **Modificare** quel commento
- Segnare quel commento **Approvato** (mostrarlo) o **Non approvato** (nasconderlo)
- Segnare quel commento come **Spam** o **Non Spam**

### Chiusura dei thread di commento

I moderatori e gli amministratori possono bloccare, o chiudere, i thread di commento selezionando `Close Thread` nel menu a tre punti nella parte superiore dell'area dei commenti, se hanno effettuato l'accesso. Possono selezionare `Re-Open Thread` in seguito, in qualsiasi momento, per riaprire i commenti.

La chiusura di un thread di commenti impedisce nuovi commenti, ma consente comunque di votare e agli utenti di eliminare i propri commenti, se lo desiderano.

La chiusura e la riapertura dei thread di commenti influisce istantaneamente su tutti gli utenti che visualizzano il thread.

È anche possibile contrassegnare un thread come sola lettura (read-only), il che rimuove anche le opzioni di voto e eliminazione, creando una regola di personalizzazione specifica per quella pagina.

### Aggiornamento in tempo reale

Tutte queste azioni aggiorneranno immediatamente i thread di commento degli altri utenti senza che debbano ricaricare la pagina. Tuttavia, azioni di moderazione come nascondere un commento o contrassegnarlo come spam non rimuovono il commento dallo schermo del **moderatore** così che, se necessario, possano annullare rapidamente l'azione. Per indicare che un commento è nascosto, verrà evidenziato rispetto agli altri commenti (il colore dell'evidenziazione dipenderà dal motivo della rimozione).

Per esempio, dati gli utenti `A (commenter)`, `B (Moderator 1)`, e `C (Moderator 2)`.

...e lo scenario seguente:

1. `User B (Moderator 1)` nasconde un commento.
2. Per `User A (commenter)` quel commento è immediatamente nascosto.
3. Per `User C (Moderator 2)` quel commento è immediatamente nascosto.
4. Per l'utente che ha effettuato la modifica, `User B (Moderator 1)`, il commento rimane sul suo schermo, ma viene evidenziato come rimosso. Ha la possibilità di annullare la sua azione, nel qual caso gli altri utenti vedranno di nuovo l'aggiornamento in tempo reale.

---