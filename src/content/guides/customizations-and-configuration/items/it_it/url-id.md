[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Quando si visualizza una discussione di commenti, o si lascia un commento, FastComments deve sapere a quale pagina, articolo o prodotto appartengono quei commenti.

Per farlo, utilizziamo qualcosa che chiamiamo "URL ID". È un identificatore, come una stringa o un numero, oppure un URL.

Per impostazione predefinita, se non specifichi urlId, diventerà l'URL della pagina. Prendiamo l'URL della pagina corrente e lo puliamo per rimuovere eventuali parametri di marketing comuni o identificatori di tracciamento.

Nel caso di integrazioni di terze parti, come WordPress, il nostro plugin solitamente utilizzerà l'identificatore che rappresenta le informazioni correnti visualizzate come URL ID, per esempio l'id dell'articolo/pagina.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Un elemento a cui faremo spesso riferimento in questo documento è l'<a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interfaccia di personalizzazione del widget</a>.

Questa interfaccia può essere usata per apportare molte modifiche al widget dei commenti senza usare codice.

Quando si crea una regola di personalizzazione, spesso vorremo che si applichi a tutte le pagine del nostro sito. Tuttavia, in alcuni casi vogliamo personalizzare il widget dei commenti su una pagina in particolare, sia per applicare uno stile personalizzato, sia magari per rendere anonimi i commenti di quella pagina. Potresti anche, per esempio, far apparire i commenti in tempo reale immediatamente su alcune pagine, mentre su altre nasconderli dietro pulsanti di notifica.

Tutto ciò è possibile tramite il campo di input URL ID in questa pagina, che appare come segue:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

Il valore in questo campo dovrebbe corrispondere al parametro *urlId* passato al widget dei commenti. Se vuoi che la tua regola di personalizzazione sia indipendente da *urlId*, lascia questo campo vuoto o inserisci *.

A partire dal 2023 il campo `URL ID` nella personalizzazione del widget accetta ora anche pattern! Ad esempio puoi avere `*/blog/*` per aggiungere uno stile specifico al tuo blog e `*/store/*` per avere uno stile specifico per il tuo store, il tutto utilizzando lo stesso dominio.

### Avvertenze

1. Se la tua pagina ha parametri hash (come example.com#page-1) - questo diventerà parte dell'URL ID, per impostazione predefinita.
2. Durante le migrazioni, ad esempio da WordPress a Gatsby, potresti dover migrare i valori di URL ID dei commenti dopo la migrazione iniziale. Per questo, contattaci.