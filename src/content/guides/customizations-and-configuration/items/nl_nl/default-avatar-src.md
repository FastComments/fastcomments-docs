[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Wanneer een gebruiker voor het eerst een reactie plaatst met FastComments, proberen we hun avatar op te halen van <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Als we echter geen avatar vinden, of de gebruiker er nooit een instelt in zijn account, tonen we een statische standaardavatarafbeelding.

Om je eigen statische avatarafbeelding op te geven, kun je de *defaultAvatarSrc* instelling gebruiken.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Overschrijf de standaardavatar'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de widget-aanpassingspagina, zie de sectie "Standaardavatar".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; alt='Standaardavatarsectie van de widget-aanpassingspagina, waar je de URL van de fallback-avatarafbeelding instelt'; title='Aanpassen van de standaardavatar' app-screenshot-end]

Let op dat het definiëren van de avatar voor een specifieke gebruiker, bijvoorbeeld met SSO, in een eigen sectie wordt behandeld.