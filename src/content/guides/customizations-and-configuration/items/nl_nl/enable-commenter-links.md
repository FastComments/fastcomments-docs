[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Standaard vraagt FastComments alleen om de opmerking, de gebruikersnaam en het e-mailadres van de gebruiker.

In sommige situaties wilt u echter dat de gebruiker een link naar zijn of haar eigen blog of website achterlaat.

We kunnen het tonen van een extra invoerveld voor de website-URL van de gebruiker inschakelen door de **enableCommenterLinks**-vlag op true te zetten:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Commentaarlinks inschakelen'; code-example-end]

Wanneer die URL wordt opgegeven, wordt het account van de gebruiker bijgewerkt en zal hun gebruikersnaam op alle eerdere en toekomstige reacties naar deze URL verwijzen.

Dit kan zonder code worden aangepast op de widget-aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Widget-aanpassingspagina met het selectievakje voor commentaarlinks aangevinkt om een website-URL-veld toe te voegen aan het reactieformulier'; title='Commentaarlinks inschakelen' app-screenshot-end]