[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments mostrerà i distintivi dell'utente solo nei commenti dell'utente all'interno del thread dei commenti.

Tuttavia, è possibile mostrare i distintivi degli utenti accanto al loro nome sopra il modulo di commento abilitando questa funzione nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Questo mostrerà i distintivi dell'utente accanto al suo nome nell'area della barra superiore, mettendo in evidenza i suoi risultati e il suo stato quando sta componendo un commento.

Nota che questa funzione deve essere abilitata nell'interfaccia di personalizzazione del widget per funzionare. Puoi opzionalmente impostare il flag **showBadgesInTopBar** su false nella configurazione del tuo codice per disabilitarla selettivamente anche quando è attivata a livello di server:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]