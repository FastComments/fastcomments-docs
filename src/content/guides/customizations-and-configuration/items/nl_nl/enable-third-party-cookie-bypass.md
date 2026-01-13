[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Voor authenticatie is FastComments afhankelijk van cookies van derden die in uw browser ingeschakeld zijn. Zonder deze cookies moeten gebruikers altijd hun e-mailadres achterlaten om te kunnen reageren (tenzij het e-mailinvoerveld verborgen is), en hun reacties zullen standaard altijd als niet-geverifieerd worden weergegeven.

Om dit te omzeilen, kunt u de omzeiling voor cookies van derden inschakelen. 

Wanneer deze instelling is ingeschakeld, verschijnt er een klein pop-upvenster dat een bericht toont dat de gebruiker wordt ingelogd. Dit pop-upvenster verschijnt telkens wanneer de gebruiker interactie heeft met de commentaarwidget; bijvoorbeeld wanneer ze een reactie achterlaten.

We kunnen dit in code doen door de vlag **enableThirdPartyCookieBypass** op true te zetten:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

We kunnen dit ook instellen via de Widget-aanpassings-UI, onder `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]