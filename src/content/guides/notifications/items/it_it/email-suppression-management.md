Quando le email inviate da FastComments rimbalzano o vengono contrassegnate come spam dal destinatario, il provider di posta aggiunge quell'indirizzo a una lista di soppressione. FastComments sincronizza queste liste di soppressione quotidianamente, in modo che ulteriori email non vengano inviate agli indirizzi che non possono riceverle.

Gli utenti e i moderatori con indirizzi email soppressi non riceveranno alcuna notifica via email, incluse le notifiche di risposta, le notifiche di menzione, gli avvisi per gli amministratori e le email di riepilogo. Un badge rosso "Email Suppressed" apparirà accanto agli utenti e ai moderatori interessati nell'interfaccia di amministrazione.

#### Visualizzazione delle Email Sopprese

Gli amministratori del tenant con il permesso Manage Data possono visualizzare le email soppresse nella pagina
[Suppressed Emails](https://fastcomments.com/auth/my-account/suppressed-emails), sotto Manage Data.

La pagina mostra una tabella di tutti gli indirizzi email soppressi associati agli utenti, moderatori e commentatori del tuo tenant. Puoi filtrare per indirizzo email utilizzando il campo di ricerca.

#### Rimozione di una Soppressione

Per rimuovere una soppressione, fai clic sul pulsante **Remove** accanto alla voce nella tabella. Verrai indirizzato a una pagina di conferma che mostra i dettagli della soppressione. Fai clic su **Confirm Removal** per procedere.

Quando una soppressione viene rimossa, FastComments contatta il provider di posta per sbloccare l'indirizzo e rimuove il flag di soppressione da tutti i record utente e moderatore associati.

#### Limiti di Frequenza

Per prevenire abusi, le rimozioni sono limitate in frequenza:

- Ogni indirizzo email può essere sbloccato solo una volta ogni 30 giorni.
- Ogni tenant può effettuare fino a 5 rimozioni per mese solare.

Se una soppressione riappare dopo la rimozione, significa che l'indirizzo email ha di nuovo rimbalzato o è stato segnalato come spam. In tal caso, il problema di deliverability sottostante dovrebbe essere risolto prima di tentare un'altra rimozione.