[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments consente agli utenti di caricare immagini. Quando un utente clicca quell'immagine, FastComments, per impostazione predefinita, apre una nuova scheda per mostrarla a grandezza intera. Impostando questo flag su true si disabilita questo comportamento:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Se non prevedi di intercettare tu stesso il clic sull'immagine (vedi [onImageClicked](#callbacks)), consigliamo di combinare questa impostazione con qualche regola di stile per rimuovere l'aspetto che l'immagine sia cliccabile.

---