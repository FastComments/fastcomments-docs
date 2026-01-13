[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments è progettato per essere personalizzato. Il widget dei commenti viene eseguito all'interno di un iframe per motivi di sicurezza, quindi per applicare
stili personalizzati è necessario seguire uno dei due approcci.

Il primo, il più semplice e quello che preferiamo, è utilizzare la [pagina di personalizzazione del widget](https://fastcomments.com/auth/my-account/customize-widget).

Nella pagina di personalizzazione del widget, vedere la sezione "Mostra opzioni avanzate", sotto la quale c'è un'area etichettata "CSS personalizzato":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Questo approccio ha alcuni vantaggi:
1. Il CSS inserito viene minimizzato prima di essere inviato all'utente e la formattazione viene mantenuta coerente nell'interfaccia di modifica.
2. Si ottengono tutti i vantaggi dell'interfaccia di personalizzazione del widget, ad esempio la possibilità di personalizzare facilmente il widget dei commenti in modo diverso per siti differenti.
3. Quando apportiamo modifiche al widget dei commenti, il tuo stile personalizzato verrà testato come parte del nostro processo di rilascio.

Il secondo approccio consiste nel specificare il parametro **customCSS** nella configurazione del widget, come segue:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Tuttavia, questo presenta *limitazioni*:
1. Esiste un limite alla quantità di CSS personalizzato che può essere passato prima che i nostri server rifiutino la richiesta, a causa della dimensione degli header.
2. Devi gestire il CSS personalizzato nella tua infrastruttura e nel tuo sistema di build. Questo può essere anche un vantaggio anziché uno svantaggio.
3. C'è un sovraccarico aggiuntivo nell'inviare il CSS personalizzato sulla rete **due volte** in questo caso d'uso, poiché deve essere inviato ai nostri server e poi inviato di nuovo nel contenuto dell'iframe. Tuttavia, per la maggior parte delle dimensioni dei payload, questo non è percettibile.
4. Una ottimizzazione comune è minimizzare il CSS per ridurne la dimensione sulla rete, ma con questo approccio dovrai gestirlo tu.
5. Il tuo CSS personalizzato non verrà testato quando apportiamo modifiche.

### File CSS esterni

Puoi dire al widget di recuperare un file esterno usando `@import`!

Si raccomanda di inserire il `@import` in una regola di personalizzazione. In questo modo, se dovessimo mai dover apportare una modifica al widget dei commenti, possiamo utilizzare i nostri strumenti di automazione
per verificare la tua configurazione. Ad esempio, creeresti una regola di personalizzazione nell'interfaccia di personalizzazione del widget, cliccheresti su `Avanzate` e inseriresti in `CSS personalizzato`:

    @import url(https://example.com/styles.css);

#### Nel codice - Non consigliato

Puoi anche caricare un file CSS esterno tramite la proprietà `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Tuttavia, ricorda che il tuo CSS non potrà essere testato da noi se procedi in questo modo. 

### Styling della finestra profilo utente

Anche i modal dei profili utente possono essere stilizzati con CSS personalizzato. Tuttavia, per garantire che lo stile personalizzato venga applicato ai profili utente, tutti i selettori CSS devono essere prefissati con `.user-profile`. Senza questo prefisso, lo stile personalizzato verrà ignorato per i modal dei profili utente.

Per esempio:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Compatibilità retroattiva

Da FastComments sappiamo che i nostri clienti personalizzano il widget dei commenti. È così per progettazione: l'ultima cosa che vogliamo è che il nostro prodotto provochi incoerenze di design nel tuo prodotto.

Poiché questa è una parte importante del nostro prodotto, abbiamo una pipeline di build che ci permette di rivedere le modifiche al widget dei commenti, per cliente, ad ogni rilascio.

Se troviamo problemi minori, aggiorneremo il tuo account per garantire che il rilascio proceda senza intoppi. Se vediamo cambiamenti che interrompono significativamente il funzionamento, questo ci permette di fermare il rilascio.