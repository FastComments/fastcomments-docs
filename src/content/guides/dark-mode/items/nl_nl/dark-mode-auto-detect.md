Standaard zal FastComments automatisch detecteren of uw site een donkere achtergrond heeft op basis van de "afstand van zwart" in de kleurencirkel.

Onze producten doen hun best hierbij, maar er zijn bijna oneindig veel kleuren in het kleurenwiel, en er kunnen scenario's zijn waarin de applicatie ervoor kiest om donkere modus te gebruiken wanneer dit niet geschikt is, en vice versa. Deze documentatie behandelt hoe u meer gedetailleerde controle hierover kunt krijgen.

#### Technische details

We detecteren donkere modus door de elementen op de pagina omhoog te doorlopen vanaf de reactie-widget, op zoek naar een donkere achtergrond wanneer de widget voor het eerst wordt geladen.

Om donkere modus na deze stap te schakelen, moet u de widget aanroepen om de configuratie bij te werken. Dit wordt behandeld in de sectie `Handmatige configuratie`.
