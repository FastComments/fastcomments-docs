---
Con FastComments, un commento Approvato è un commento **visibile**. Per nascondere un commento, dovresti revocarne l'approvazione.

I commenti possono essere approvati automaticamente, oppure i moderatori possono approvare manualmente ogni commento. C'è anche l'opzione di
richiedere l'approvazione solo del primo commento di un utente - in tal caso i commenti successivi vengono approvati automaticamente e non richiedono moderazione.

FastComments ha il concetto di Verificato vs Non verificato. I commenti Verificati sono o pubblicati in una sessione verificata via email (l'utente ha effettuato il login completo, o usa SSO)
oppure sono stati pubblicati in stato Non verificato e poi successivamente verificati manualmente via email.

La nozione di Verificato può essere completamente nascosta, se desiderato, tramite regole di personalizzazione.

Richiedere l'approvazione manuale dei commenti non verificati può aiutare contro lo spam, poiché i bot raramente verificano i loro commenti via email. In questo caso
vorrai abilitare l'approvazione automatica dei commenti ma attivare `Only Auto Approve Verified Comments`.

Tutto questo può essere configurato dagli amministratori da [Impostazioni di moderazione](https://fastcomments.com/auth/my-account/moderate-comments/settings).

---