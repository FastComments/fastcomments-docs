[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Som standard tillader FastComments brugere at uploade billeder. Når en bruger klikker på det billede, åbner FastComments som standard,
en ny fane for at vise billedet i fuld størrelse. At sætte denne indstilling til true deaktiverer denne adfærd:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Hvis du ikke planlægger at fange klik på billedet selv (se [onImageClicked](#callbacks)), anbefaler vi, at dette kombineres med noget styling
for at fjerne indtrykket af, at billedet kan klikkes på.

---