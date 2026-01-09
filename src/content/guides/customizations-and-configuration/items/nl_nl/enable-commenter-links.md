[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Standaard vraagt FastComments de gebruiker alleen om hun reactie, hun gebruikersnaam en hun e-mail.

In sommige situaties wilt u echter dat de gebruiker een link naar hun eigen blog of website achterlaat.

We kunnen het tonen van een extra invoerveld voor de website-URL van de gebruiker inschakelen door de vlag **enableCommenterLinks** op true te zetten:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Wanneer die URL is opgegeven, wordt het account van de gebruiker bijgewerkt en zal hun gebruikersnaam bij alle eerdere en toekomstige reacties naar deze URL linken.

Dit kan zonder code worden aangepast op de pagina voor het aanpassen van de widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---