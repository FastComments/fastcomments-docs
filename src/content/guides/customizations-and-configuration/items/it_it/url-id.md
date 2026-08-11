[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Durante il rendering di una discussione di commenti, o lasciando un commento, FastComments deve sapere a quale pagina, articolo o prodotto appartengono quei commenti.

Per fare ciò, utilizziamo qualcosa che chiamiamo "URL ID". È un identificatore, come una stringa o un numero, oppure un URL.

Per impostazione predefinita, se non specifichi l'urlId, verrà utilizzato l'URL della pagina. Prenderemo l'URL della pagina corrente e lo puliremo per rimuovere eventuali parametri di marketing comuni o identificatori di tracciamento.

Nel caso di integrazioni di terze parti, come WordPress, il nostro plugin utilizzerà solitamente l'identificatore che rappresenta le informazioni attualmente visualizzate come URL ID, ad esempio l'ID dell'articolo/pagina.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definire un URL ID personalizzato'; code-example-end]

Una cosa a cui faremo spesso riferimento in questo documento è l'<a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interfaccia di personalizzazione del widget</a>.

Questa interfaccia può essere utilizzata per apportare molte modifiche al widget dei commenti senza usare codice.

Quando si crea una regola di personalizzazione, spesso vogliamo che si applichi a tutte le pagine del nostro sito. Tuttavia, in alcuni casi desideriamo personalizzare il widget dei commenti su una pagina specifica, ad esempio per applicare uno stile personalizzato o per rendere i commenti di quella pagina anonimi. Si potrebbe anche, per esempio, far apparire i commenti in tempo reale subito su alcune pagine, nascondendoli sotto pulsanti di notifica su altre.

Tutto ciò è possibile tramite il campo di input URL ID su questa pagina, che appare come segue:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Campo URL ID usato per limitare una regola di personalizzazione a una pagina, o a un modello come */blog/*'; title='Input URL ID nella pagina di personalizzazione del widget' app-screenshot-end]

Il valore in questo campo dovrebbe corrispondere al parametro *urlId* passato al widget dei commenti. Se desideri che la tua regola di personalizzazione sia indipendente da *urlId*, lascia questo campo vuoto o inserisci *.

A partire dal 2023 il campo `URL ID` nella personalizzazione del widget accetta anche i modelli! Ad esempio potresti avere `*/blog/*` per aggiungere uno stile specifico al tuo blog e `*/store/*` per avere uno stile specifico al tuo negozio, il tutto usando lo stesso dominio.

### Problemi comuni

1. Se la tua pagina ha parametri hash (come example.com#page-1) - questo diventerà parte dell'URL ID, per impostazione predefinita.  
2. Durante le migrazioni, ad esempio da WordPress a Gatsby, potresti dover migrare i valori dei commenti URL ID dopo la migrazione iniziale. Per questo, contattaci.

---