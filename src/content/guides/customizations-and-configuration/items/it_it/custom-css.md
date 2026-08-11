[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments è progettato per essere personalizzato. Il widget dei commenti stesso gira all'interno di un iframe per motivi di sicurezza, quindi per applicare uno stile personalizzato devi seguire una delle due modalità.

Il primo, il più semplice, e il nostro preferito, è utilizzare la [pagina di personalizzazione del widget](https://fastcomments.com/auth/my-account/customize-widget).

Nella pagina di personalizzazione del widget, vedi la sezione "Mostra opzioni avanzate", sotto la quale c'è un'area etichettata "CSS personalizzato":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Editor CSS personalizzato sotto Opzioni avanzate nella pagina di personalizzazione del widget'; title='Area di input CSS personalizzato' app-screenshot-end]

Questo approccio ha alcuni vantaggi:
1. Il CSS inserito viene minificato prima di essere inviato all'utente, e la formattazione viene mantenuta coerente nell'interfaccia di modifica.
2. Ottieni tutti i vantaggi dell'interfaccia di personalizzazione del widget, ad esempio personalizzando facilmente il widget dei commenti in modo diverso per siti diversi.
3. Quando apportiamo modifiche al widget dei commenti, il tuo stile personalizzato verrà testato come parte del nostro processo di rilascio.

Il secondo approccio è specificare il parametro **customCSS** nella configurazione del widget, come segue:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passare CSS personalizzato'; code-example-end]

Tuttavia, questo ha *limiti*:
1. C'è un limite a quanta CSS personalizzata può essere passata prima che i nostri server rifiutino la richiesta, a causa della dimensione delle intestazioni.
2. Devi gestire il CSS personalizzato nella tua infrastruttura e nel sistema di build. Questo può anche essere un vantaggio piuttosto che uno svantaggio.
3. C'è un overhead aggiuntivo nell'inviare il CSS personalizzato sulla rete **due** volte in questo caso d'uso, poiché deve essere inviato ai nostri server e poi restituito nel contenuto dell'iframe. Tuttavia, per la maggior parte delle dimensioni del payload, ciò non è percepibile.
4. Una comune ottimizzazione è minificare il CSS per ridurne le dimensioni sulla rete, ma con questo approccio dovrai gestirlo tu.
5. Il tuo CSS personalizzato non verrà testato quando apportiamo modifiche.

### File CSS esterni

Puoi far sì che il widget recuperi un file esterno usando `@import`!

È consigliato inserire il `@import` in una regola di personalizzazione. In questo modo, se dovessimo mai dover modificare il widget dei commenti, possiamo utilizzare i nostri strumenti di automazione per verificare la tua configurazione. Quindi, ad esempio, creeresti una regola di personalizzazione nell'interfaccia di personalizzazione del widget, cliccheresti su `Avanzate` e inseriresti nel `CSS personalizzato`:

    @import url(https://example.com/styles.css);

#### In codice - non consigliato

Puoi anche caricare un file CSS esterno tramite la proprietà `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'File CSS esterno'; code-example-end]

Tuttavia, ricorda che il tuo CSS non potrà essere testato da noi se lo fai in questo modo. 

### Stile del modal del profilo utente

I modal del profilo utente possono anche essere stilizzati con CSS personalizzato. Tuttavia, per garantire che lo stile personalizzato venga applicato ai profili utente, tutti i selettori CSS devono essere prefissati con `.user-profile`. Senza questo prefisso, lo stile personalizzato verrà ignorato per i modal del profilo utente.

Ad esempio:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'CSS profilo utente'; code-example-end]

### Compatibilità retroattiva

In FastComments, sappiamo che i nostri clienti personalizzano il widget dei commenti. È così progettato - l'ultima cosa che vogliamo è che il nostro prodotto causi incoerenze di design nel tuo prodotto.

Poiché è una parte importante del nostro prodotto, disponiamo di una pipeline di build che ci consente di revisionare le modifiche al widget dei commenti, per cliente, ad ogni rilascio.

Se troviamo problemi minori, aggiorneremo il tuo account per garantire che il nostro rilascio proceda senza intoppi. Se vediamo cambiamenti importanti che interrompono il funzionamento, questo ci permette di fermare il rilascio.