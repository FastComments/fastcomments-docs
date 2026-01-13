[related-parameter-start name = 'defaultAvatarSrc'; type = 'string'; related-parameter-end]

Wanneer een gebruiker voor het eerst met FastComments een reactie plaatst, proberen we hun avatar op te halen van <a href="http://gravatar.com/" target="_blank">http://gravatar.com/</a>.

Als we echter geen avatar vinden, of de gebruiker er nooit een in zijn/haar account instelt, tonen we een statische standaardavatar-afbeelding.

Om uw eigen statische avatar-afbeelding op te geven, kunt u de *defaultAvatarSrc*-instelling gebruiken.

[code-example-start config = {defaultAvatarSrc: "https://example.com/some-image.png"}; linesToHighlight = [6]; title = 'Override The Default Avatar'; code-example-end]

Dit kan ook zonder code. Op de pagina voor widget-aanpassing, zie de "Standaardavatar" sectie.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-avatar'; title='Customizing The Default Avatar' app-screenshot-end]

Houd er rekening mee dat het definiëren van de avatar voor een specifieke gebruiker, zoals bij SSO, in een aparte sectie wordt behandeld.

---