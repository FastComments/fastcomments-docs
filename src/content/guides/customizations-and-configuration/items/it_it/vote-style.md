[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Per impostazione predefinita, FastComments visualizza le opzioni di voto come frecce su e giù, consentendo agli utenti di votare un commento positivamente o negativamente.

Tuttavia, è possibile modificare lo stile della barra degli strumenti di voto. Le opzioni attuali sono i pulsanti predefiniti Su/Giù o l'utilizzo di un meccanismo di voto in stile Cuore.

Utilizziamo il flag **voteStyle** come segue:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Abilita pulsante a cuore'; code-example-end]

Consigliamo vivamente di farlo senza codice, poiché abilita anche le convalide lato server. Nella pagina di personalizzazione del widget, vedere la sezione "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Impostazione dello stile di voto nella pagina di personalizzazione del widget, che offre frecce su e giù o voto a cuore'; title='Cambia stile di voto' app-screenshot-end]

Il voto può anche essere disabilitato, vedere `Disable Voting` sopra le opzioni di stile.