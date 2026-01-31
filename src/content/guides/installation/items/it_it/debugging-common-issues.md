Qui ci sono alcuni sintomi che riscontriamo frequentemente e le soluzioni comuni. 

### Messaggio "Questa è una demo"

Questo viene mostrato quando hai copiato il codice del widget dalla nostra homepage, che utilizza il nostro tenant demo. Per usare il tuo tenant, copia il codice del widget da [qui](https://fastcomments.com/auth/my-account/get-acct-code).

### Errore "FastComments non riesce a caricarsi su questo dominio"

FastComments deve sapere quali domini sono di tua proprietà per autenticare le richieste associate al tuo account. [Consulta la nostra documentazione](/guide-multiple-sites.html#add-domains-to-account) per vedere come risolvere questo errore (aggiungi semplicemente il sottodominio + dominio esatti al tuo account).

Nota che questo dovrebbe verificarsi solo dopo la fine del periodo di prova. Durante il periodo di prova, tutte le richieste da nuovi domini verranno automaticamente aggiunte al tuo account.

### Commenti migrati non visualizzati per installazioni personalizzate

Di solito ciò accade quando i commenti importati sono legati a un `Page ID`, e tu stai passando un URL (o nessun valore, nel qual caso viene usato l'URL della pagina).

Puoi fare il debug [esportando i tuoi commenti](https://fastcomments.com/auth/my-account/manage-data/export) e visualizzando la colonna `URL ID` (attualmente Colonna `B`).

Assicurati che i valori che vedi nella colonna `URL ID` siano gli stessi valori che stai passando alla configurazione del widget come parametro `urlId`.

Per ulteriori spiegazioni, prova a leggere la nostra documentazione [Come i commenti sono legati alle pagine e agli articoli](/guide-customizations-and-configuration.html#url-id).

Se tutto il resto fallisce, [contattaci](https://fastcomments.com/auth/my-account/help).

### Il widget dei commenti non viene visualizzato

Se il widget dei commenti non viene visualizzato, controlla la console sviluppatore di Chrome per eventuali errori.

Per la maggior parte delle errate configurazioni, il widget dei commenti mostrerà almeno un errore sulla pagina se riesce a caricarsi. Non vedere nulla è solitamente un'indicazione di un errore di scripting.

### La configurazione desiderata non funziona come previsto

Prova la nostra [estensione Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) per vedere quale configurazione viene passata al widget dei commenti. Se tutto fallisce, fai uno screenshot di ciò che indica l'estensione Chrome e [contattaci](https://fastcomments.com/auth/my-account/help).

### Commenti mancanti sulla stessa URL con differenti Hash Bang

Per impostazione predefinita, FastComments utilizza l'URL della pagina per il "bucket" in cui i commenti vengono archiviati. Se i tuoi URL includono `#hashbangs`, e questi `#hashbangs` non dovrebbero far parte dell'identificatore che identifica un thread di commenti, possiamo semplicemente ignorare il valore dell'hash bang, per esempio:

[inline-code-attrs-start title = 'Esempio: ignorare gli hash bang'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Nota che dopo aver effettuato questa modifica, sarà necessario eseguire una migrazione per i commenti esistenti. [Per questo, contattaci.](https://fastcomments.com/auth/my-account/help)

### Parametri di query URL che influenzano il widget

Per impostazione predefinita, FastComments utilizza l'URL della pagina per il "bucket" in cui i commenti vengono archiviati. Se i tuoi URL includono parametri di query che non dovrebbero far parte dell'identificatore che identifica un thread di commenti, possiamo semplicemente ignorarli, per esempio:

[inline-code-attrs-start title = 'Ignora i parametri di query'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Nota che dopo aver effettuato questa modifica, sarà necessario eseguire una migrazione per i commenti esistenti. [Per questo, contattaci.](https://fastcomments.com/auth/my-account/help)

### Non ricevere le email

In FastComments dedichiamo molti sforzi per garantire che l'invio delle email sia il più affidabile possibile. Tuttavia, alcuni provider di posta elettronica sono notoriamente difficili da raggiungere in modo affidabile. Controlla la cartella spam per i messaggi provenienti da fastcomments.com.

Se [ci contatti](https://fastcomments.com/auth/my-account/help) di solito possiamo fornire maggiori informazioni sul motivo per cui potresti non ricevere le nostre email.

---