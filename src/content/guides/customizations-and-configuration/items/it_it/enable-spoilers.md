[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Possiamo abilitare il supporto per i spoiler impostando il flag **enableSpoilers** su true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Questo può essere fatto anche senza codice. Nella pagina di personalizzazione del widget, vedere l'opzione "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Pagina di personalizzazione del widget con la casella Enable Spoilers selezionata per aggiungere il pulsante SPOILER all\'editor'; title='Abilita Spoiler' app-screenshot-end]

Quando il testo è evidenziato e il pulsante `SPOILER` ora visibile viene cliccato, il testo verrà mascherato finché l'utente non vi passa sopra con il mouse. Per la modalità scura facciamo la stessa cosa, con colori diversi che si adattano meglio alla modalità scura.

Questo è anche compatibile con l'editor WYSIWYG.