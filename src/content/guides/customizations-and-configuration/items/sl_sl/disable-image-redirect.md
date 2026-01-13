[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Privzeto FastComments dovoljuje uporabnikom nalaganje slik. Ko uporabnik klikne to sliko, FastComments privzeto odpre nov zavihek, da prikaže sliko v celoti. Nastavitev te zastavice na true onemogoči to vedenje:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Če ne nameravate sami prestrezati klika na sliko (glejte [onImageClicked](#callbacks)), priporočamo, da to kombinirate z nekim stiliranjem
da odstranite videz, da je slika klikljiva.