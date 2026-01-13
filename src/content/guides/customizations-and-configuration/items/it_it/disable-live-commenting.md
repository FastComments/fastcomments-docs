[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments avrà i commenti live abilitati.

Ciò significa che ogni visualizzatore del thread dei commenti dovrebbe vedere lo stesso contenuto.

Ad esempio, se viene aggiunto un commento, quel commento dovrebbe essere visibile. Se un commento viene modificato o rimosso,
allora quei commenti saranno modificati o rimossi per tutti i visualizzatori del thread. Lo stesso vale per i voti e per tutte le azioni di moderazione.

Tuttavia, possiamo disabilitare questo:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Questo può anche essere fatto senza codice. Nella pagina di personalizzazione del widget, vedere la sezione "Disabilita commenti live".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]