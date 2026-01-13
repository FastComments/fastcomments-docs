Ecco alcuni sintomi che incontriamo frequentemente e soluzioni comuni.

### Messaggio "This is a demo"

Questo viene mostrato quando hai copiato il codice del widget dalla nostra home page, che utilizza il nostro tenant demo. Per usare il tuo tenant, copia il codice del widget da [qui](https://fastcomments.com/auth/my-account/get-acct-code).

### Errore "FastComments cannot load on this domain"

FastComments deve sapere quali domini ti appartengono per autenticare le richieste associate al tuo account. [Consulta la nostra documentazione](/guide-multiple-sites.html#add-domains-to-account) per vedere come risolvere questo errore (aggiungi semplicemente il sottodominio esatto + dominio al tuo account).

Nota che questo dovrebbe verificarsi solo dopo la fine del periodo di prova. Durante il periodo di prova, qualsiasi richiesta da nuovi domini verrà automaticamente aggiunta al tuo account.

### I commenti migrati non vengono mostrati per installazioni personalizzate

Di solito questo accade quando i commenti importati sono legati a un `Page ID`, e stai passando un URL (o nessun valore, nel qual caso viene usato di default l'URL della pagina).

Puoi fare debug [esportando i tuoi commenti](https://fastcomments.com/auth/my-account/manage-data/export) e visualizzando la colonna `URL ID` (attualmente la colonna `B`).

Assicurati che i valori che vedi nella colonna `URL ID` siano gli stessi valori che stai passando alla configurazione del widget come parametro `urlId`.

Per ulteriori spiegazioni, prova a leggere la nostra [documentazione su come i commenti sono legati a pagine e articoli](/guide-customizations-and-configuration.html#url-id).

Se tutto il resto fallisce, [contattaci](https://fastcomments.com/auth/my-account/help).

### Il widget dei commenti non viene mostrato

Se il widget dei commenti non viene mostrato, controlla la console sviluppatore di Chrome per errori.

Per la maggior parte delle configurazioni errate, il widget dei commenti mostrerà almeno un errore sulla pagina se riesce a caricarsi. Non vedere nulla è di solito un'indicazione di un errore di scripting.

### La configurazione desiderata non funziona come previsto

Prova la nostra [estensione Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) per vedere quale configurazione viene passata al widget dei commenti. Se tutto il resto fallisce, fai uno screenshot di ciò che dice l'estensione Chrome e [contattaci](https://fastcomments.com/auth/my-account/help).

### Commenti mancanti sullo stesso URL con hash bang diverso

Per impostazione predefinita, FastComments userà l'URL della pagina per il "bucket" dove sono memorizzati i commenti. Se i tuoi URL includono `#hashbangs`, e questi `#hashbangs` non dovrebbero far parte dell'identificatore che identifica un thread di commenti, possiamo semplicemente ignorare il valore dell'hash bang, per esempio:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Nota che dopo aver fatto questa modifica, dovrà essere eseguita una migrazione per i commenti esistenti. [Per questo, contattaci.](https://fastcomments.com/auth/my-account/help)

### I parametri di query dell'URL influenzano il widget

Per impostazione predefinita, FastComments userà l'URL della pagina per il "bucket" dove sono memorizzati i commenti. Se i tuoi URL includono parametri di query che non dovrebbero far parte dell'identificatore che identifica un thread di commenti, possiamo semplicemente ignorarli, per esempio:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Nota che dopo aver fatto questa modifica, dovrà essere eseguita una migrazione per i commenti esistenti. [Per questo, contattaci.](https://fastcomments.com/auth/my-account/help)

### Non ricevi email

In FastComments, mettiamo molto impegno per assicurare che la nostra consegna di email sia il più affidabile possibile. Tuttavia, alcuni provider di posta elettronica sono notoriamente difficili da raggiungere in modo affidabile. Controlla la tua cartella spam per messaggi da fastcomments.com.

Se [ci contatti](https://fastcomments.com/auth/my-account/help) possiamo di solito fornire maggiori informazioni sul perché potresti non vedere email da noi.
