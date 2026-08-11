[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

I commenti possono essere bloccati in modo che non vengano lasciati nuovi commenti o voti impostando il flag readonly su true.

I commenti non potranno inoltre essere modificati o eliminati.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget, per un intero dominio o pagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Impostazione per impedire nuove risposte nella pagina di personalizzazione del widget, che blocca un thread per un dominio o una pagina'; title='Rendere il thread dei commenti in sola lettura' app-screenshot-end]

## Aggiornamento!

A partire da novembre 2022, i thread possono essere bloccati o sbloccati **in tempo reale** dagli amministratori e moderatori tramite il menu a tre puntini sopra l'area di risposta.

Questo impedirà nuovi commenti, consentendo comunque la votazione e permettendo agli utenti di eliminare i propri commenti se lo desiderano, mentre `readonly` non consente queste funzionalità. 

Ciò corrisponde al campo `isClosed` nell'API `Page`.