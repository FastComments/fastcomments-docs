[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments avrà i commenti in tempo reale abilitati.

Ciò significa che ogni visualizzatore della discussione dei commenti dovrebbe vedere lo stesso contenuto.

Ad esempio, se viene aggiunto un commento, quel commento dovrebbe apparire. Se un commento viene modificato o rimosso,
allora quei commenti saranno modificati o rimossi per tutti i visualizzatori della discussione. Lo stesso vale per i voti e per tutte le azioni di moderazione.

Tuttavia, possiamo disabilitarlo:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disabilita i commenti in tempo reale'; code-example-end]

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, vedere la sezione "Disabilita i commenti in tempo reale".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Sezione Disabilita i commenti in tempo reale della pagina di personalizzazione del widget, che disattiva gli aggiornamenti in tempo reale della discussione'; title='Disabilita i commenti in tempo reale' app-screenshot-end]