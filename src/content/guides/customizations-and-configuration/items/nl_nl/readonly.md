[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Commentaar kan worden vergrendeld zodat er geen nieuwe reacties of stemmen kunnen worden geplaatst door de readonly‑vlag op true te zetten.

Reacties kunnen ook niet meer worden bewerkt of verwijderd.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Dit kan zonder code worden aangepast, op de widget‑aanpassingspagina, voor een heel domein of een pagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Voorkom nieuwe antwoorden-instelling op de widget‑aanpassingspagina, die een thread vergrendelt voor een domein of pagina'; title='De commentaarthread readonly maken' app-screenshot-end]

## Update!

Vanaf november 2022 kunnen threads **live** worden vergrendeld of ontgrendeld door beheerders en moderators via het drie‑punt‑menu boven het antwoordgebied.

Dit voorkomt nieuwe reacties, terwijl stemmen nog wel mogelijk blijft en gebruikers hun reacties kunnen verwijderen indien gewenst, terwijl `readonly` deze mogelijkheden niet toestaat. 

Dit komt overeen met het `isClosed`‑veld in de `Page`‑API.