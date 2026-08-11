---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments mostrerà i badge degli utenti solo sui loro commenti all'interno del thread dei commenti.

Tuttavia, possiamo mostrare i badge degli utenti accanto al loro nome sopra il modulo dei commenti abilitando questa funzionalità nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Casella di controllo per mostrare i badge nella barra superiore nella pagina di personalizzazione del widget, posizionando i badge accanto al nome sopra il modulo dei commenti'; title='Opzione Mostra Badge nella Barra Superiore' app-screenshot-end]

Questo visualizzerà i badge dell'utente accanto al suo nome nell'area della barra superiore, rendendo i suoi risultati e il suo stato più evidenti mentre sta scrivendo un commento.

Nota che questa funzionalità deve essere abilitata nell'interfaccia di personalizzazione del widget per funzionare. Puoi opzionalmente impostare il flag **showBadgesInTopBar** su false nella configurazione del tuo codice per disabilitarla selettivamente anche quando è attivata a livello di server:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---