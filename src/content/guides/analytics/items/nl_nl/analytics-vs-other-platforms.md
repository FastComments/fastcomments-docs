U kunt merken dat onze Analytics-statistieken iets andere cijfers tonen dan bijvoorbeeld Google Ads © of vergelijkbare producten.

Voor sites met één reactie-widget per pagina zijn de cijfers van FastComments Analytics zeer nauwkeurig, en als ze onjuist zijn, zullen ze **lager** zijn dan de werkelijke waarde, maar niet meer.

Als u een SPA hebt, kunt u merken dat de FastComments Analytics-cijfers hoger zijn dan die gerapporteerd door uw marketingproducten. Dit komt omdat het marketingproduct mogelijk alleen bijhoudt wanneer de pagina niet is geladen, maar niet elke keer dat een gebruiker iets op de pagina doet dat het tonen van een nieuwe reactie-thread kan triggeren, wat zou tellen als een pagina-lading voor FastComments Analytics.

#### Technische informatie

FastComments Analytics houdt elke pagina-lading bij en vertrouwt niet op willekeurigheid als optimalisatie. Elke pagina-lading resulteert in een in-memory teller die wordt bijgewerkt in elke thread op elke server, die vervolgens periodiek wordt opgeslagen in de database via een atomaire operatie.
