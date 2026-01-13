[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Het plaatsen van reacties kan worden vergrendeld zodat er geen nieuwe reacties of stemmen geplaatst kunnen worden door de readonly-vlag op true te zetten.

Reacties kunnen ook niet worden bewerkt of verwijderd.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Dit kan zonder code worden aangepast op de widget-aanpassingspagina, voor een volledig domein, of een pagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Bijgewerkt!

Vanaf november 2022 kunnen discussiedraden door beheerders en moderators **live** worden vergrendeld of ontgrendeld via het driepuntjesmenu boven het antwoordgebied.

Dit voorkomt nieuwe reacties, terwijl stemmen nog wel mogelijk blijft en gebruikers hun reacties kunnen verwijderen indien gewenst, terwijl `readonly` deze acties niet toestaat. 

Dit komt overeen met het veld `isClosed` in de `Page` API.

---