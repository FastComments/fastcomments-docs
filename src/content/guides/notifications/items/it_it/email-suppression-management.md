Quando le email inviate da FastComments rimbalzano o vengono contrassegnate come spam dal destinatario, il provider di posta aggiunge quell'indirizzo a una
lista di soppressione. FastComments sincronizza queste liste di soppressione quotidianamente in modo che non vengano inviate ulteriori email a indirizzi che
non possono riceverle.

Gli utenti e i moderatori con indirizzi email soppressi non riceveranno alcuna notifica via email, incluse le notifiche di risposta,
le notifiche di menzione, gli avvisi amministrativi e le email di digest. Accanto agli utenti e ai moderatori interessati nell'interfaccia di amministrazione
apparirà un badge rosso "Email soppressa".

#### Viewing Suppressed Emails

Gli amministratori del tenant con il permesso Manage Data possono visualizzare le email soppresse nella
[Email soppresse](https://fastcomments.com/auth/my-account/suppressed-emails) page, sotto Manage Data.

La pagina mostra una tabella di tutti gli indirizzi email soppressi associati agli utenti, moderatori e commentatori del tuo tenant.
Puoi filtrare per indirizzo email usando il campo di ricerca.

#### Removing a Suppression

Per rimuovere una soppressione, fai clic sul pulsante **Rimuovi** accanto alla voce nella tabella. Verrai portato a una pagina di conferma
che mostra i dettagli della soppressione. Fai clic su **Conferma rimozione** per procedere.

Quando una soppressione viene rimossa, FastComments contatta il provider di posta per sbloccare l'indirizzo e cancella il flag di soppressione
da tutti i record utente e moderatore associati.

#### Rate Limits

Per prevenire abusi, le rimozioni sono soggette a limiti di frequenza:

- Ogni indirizzo email può essere rimosso dalla soppressione una sola volta ogni 30 giorni.
- Ciascun tenant può eseguire fino a 5 rimozioni per mese solare.

Se una soppressione riappare dopo la rimozione, significa che l'indirizzo email è rimbalzato o è stato segnalato come spam di nuovo. In tal caso,
il problema di deliverability sottostante dovrebbe essere risolto prima di tentare un'altra rimozione.