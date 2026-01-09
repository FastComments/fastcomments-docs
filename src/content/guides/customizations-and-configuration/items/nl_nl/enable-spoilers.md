[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

We kunnen ondersteuning voor spoilers inschakelen door de vlag **enableSpoilers** op true te zetten:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Dit kan ook zonder code worden gedaan. Op de pagina voor widget-aanpassing, zie de "Enable Spoilers" optie.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Wanneer tekst gemarkeerd is, en de nu zichtbare `SPOILER` knop wordt aangeklikt, wordt de tekst afgeschermd totdat de gebruiker er met de muis overheen gaat. Voor de donkere modus doen we hetzelfde, maar met andere
kleuren die beter bij de donkere modus passen.

Dit is ook compatibel met de WYSIWYG editor.