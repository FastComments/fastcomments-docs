[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Voor authenticatie is FastComments afhankelijk van ingeschakelde third‑party‑cookies in uw browser. Zonder deze cookies moeten gebruikers altijd hun e‑mail achterlaten om te reageren (tenzij het e‑mail invoerveld verborgen is), en hun reacties worden standaard als niet‑geverifieerd weergegeven.

Om dit te omzeilen kunt u de third‑party‑cookie‑bypass inschakelen. 

Wanneer deze instelling is ingeschakeld, veroorzaakt dit een klein pop‑upvenster dat een bericht toont waarin staat dat de gebruiker wordt ingelogd. Dit pop‑upvenster wordt getoond telkens wanneer de gebruiker interactie heeft met de commentaarwidget; bijvoorbeeld wanneer hij/zij een reactie achterlaat.

We kunnen dit in code doen door de **enableThirdPartyCookieBypass**‑vlag op true te zetten:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Third-Party Cookie Bypass inschakelen'; code-example-end]

We kunnen dit ook instellen via de Widget‑aanpassings‑UI, onder `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Widget-aanpassingspagina met het selectievakje Enable Third-Party Cookie Popup aangevinkt'; title='Third-Party Cookie Bypass inschakelen' app-screenshot-end]