[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

I commenti possono essere bloccati in modo che non possano essere lasciati nuovi commenti o voti impostando il flag readonly su true.

I commenti non potranno inoltre essere modificati o eliminati.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget, per un intero dominio o per una pagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Aggiornamento!

A partire da novembre 2022, i thread possono essere bloccati o sbloccati **in tempo reale** da amministratori e moderatori tramite il menu a tre puntini sopra l'area di risposta.

Questo impedirà nuovi commenti, pur consentendo il voto e permettendo agli utenti di eliminare i propri commenti se lo desiderano, mentre `readonly` non consente queste operazioni. 

Questo corrisponde al campo `isClosed` nell'API `Page`.

---