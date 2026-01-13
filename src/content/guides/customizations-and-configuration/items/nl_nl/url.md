[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Bij het verzenden van notificatie-e-mails, of het weergeven van opmerkingen in gebruikersinterfaces zoals de moderatiepagina, is het handig om een link te kunnen maken
van de opmerking naar de pagina waarop deze staat.

Als URL-ID niet altijd een ID is, moeten we de URL ergens anders opslaan. Daar is de "url" eigenschap voor, gedefinieerd als volgt.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Een veelvoorkomend gebruik is het koppelen van de reactiedraad aan een identificator, zoals een artikel, en vervolgens teruglinken naar een specifieke pagina, bijvoorbeeld:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

De URL wordt niet opgeschoond van gebruikelijke marketingparameters. Standaard wordt, wat de huidige pagina-URL ook is, die URL met de opmerking opgeslagen.