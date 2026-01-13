---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Possiamo abilitare il supporto agli spoiler impostando il flag **enableSpoilers** su true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Questo può anche essere fatto senza codice. Nella pagina di personalizzazione del widget, vedi l'opzione "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Quando del testo viene evidenziato e viene cliccato il pulsante `SPOILER` ora visibile, il testo sarà mascherato finché l'utente non ci passa sopra il cursore. Per la modalità scura facciamo la stessa cosa, con colori diversi che si adattano meglio alla modalità scura.

Questo è anche compatibile con l'editor WYSIWYG.

---