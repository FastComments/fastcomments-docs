[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Per impostazione predefinita, FastComments visualizza le opzioni di voto come frecce su e giù, permettendo agli utenti di effettuare un voto positivo o negativo su un commento.

È comunque possibile cambiare lo stile della barra degli strumenti di voto. Le opzioni attuali sono i pulsanti predefiniti Su/Giù, oppure utilizzare un meccanismo di voto a forma di cuore.

Usiamo il flag **voteStyle** come segue:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Consigliamo vivamente di farlo senza codice poiché abilita anche le convalide lato server. Nella pagina di personalizzazione del widget, consultare la sezione "Stile di voto".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Il voto può anche essere disabilitato, vedere `Disable Voting` sopra le opzioni di stile.