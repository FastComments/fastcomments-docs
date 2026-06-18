Standaard staat FastComments geen iframes in reacties toe. Wanneer je media-embeds inschakelt, kunnen commentatoren de embedcode (de `<iframe>`-snippet) van vertrouwde providers zoals YouTube, Vimeo, SoundCloud en Spotify plakken, en deze wordt inline in de reactie weergegeven.

Om veiligheidsredenen is dit geen configuratievlag van de widget aan de clientzijde. Het is een instelling aan de serverzijde, gevalideerd wanneer elke reactie wordt opgeslagen, dus het kan niet vanaf de pagina worden ingeschakeld. Alleen iframes die verwijzen naar een ingebouwde lijst met vertrouwde providers zijn toegestaan. Elk ander iframe wordt verwijderd.

Dit wordt gedaan zonder code, op de pagina voor het aanpassen van de widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Je eigen providers toevoegen

Als je embeds wilt toestaan van een provider die niet op de ingebouwde lijst met vertrouwde providers staat, voeg dan de hostnaam toe in het veld "Additional Embed Domains" op dezelfde pagina. Deze hostnamen zijn toegestaan naast de ingebouwde providers. De overeenkomst is exact, dus geef de volledige hostnaam op (bijvoorbeeld player.example.com). Alles wat je niet vermeldt blijft geblokkeerd.

Zowel het gewone reactieveld als de WYSIWYG-editor ondersteunen het plakken van een embed. In de WYSIWYG-editor wordt de embed ingevoegd als een verwijderbaar blok.