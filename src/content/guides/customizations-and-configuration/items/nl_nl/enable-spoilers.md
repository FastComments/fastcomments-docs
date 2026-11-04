[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

We kunnen spoiler-ondersteuning inschakelen door de vlag **enableSpoilers** op true te zetten:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Spoilers inschakelen'; code-example-end]

Dit kan ook zonder code. Zie op de widget-aanpassingspagina de optie "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Widget-aanpassingspagina met het selectievakje Enable Spoilers aangevinkt om de SPOILER-knop aan de editor toe te voegen'; title='Enable Spoilers' app-screenshot-end]

Wanneer tekst is gemarkeerd en de nu zichtbare `SPOILER`-knop wordt aangeklikt, wordt de tekst gemaskeerd totdat de gebruiker er met de muis overheen beweegt. Voor de donkere modus doen we hetzelfde, met andere
kleuren die beter passen bij de donkere modus.

Dit is ook compatibel met de WYSIWYG-editor.