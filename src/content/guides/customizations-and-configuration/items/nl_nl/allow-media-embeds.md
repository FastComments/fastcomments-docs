Standaard staat FastComments geen iframes toe in reacties. Wanneer je media‑embeds inschakelt, kunnen reageerders de embedcode (de `<iframe>`‑snippet) van vertrouwde providers zoals YouTube, Vimeo, SoundCloud en Spotify plakken, en deze wordt inline weergegeven in de reactie.

Voor de veiligheid is dit geen client‑side widget‑configuratievlag. Het is een server‑side instelling, gevalideerd wanneer elke reactie wordt opgeslagen, zodat deze niet vanaf de pagina kan worden ingeschakeld. Alleen iframes die verwijzen naar een ingebouwde lijst van vertrouwde providers zijn toegestaan. Elke andere iframe wordt verwijderd.

Dit gebeurt zonder code, op de widget‑aanpassingspagina:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Media-embedinstelling ingeschakeld op de widget-aanpassingspagina, waardoor reageerders vertrouwde iframe-embeds kunnen plakken'; title='Media-embeds toestaan' app-screenshot-end]

### Eigen providers toevoegen

Als je embeds van een provider wilt toestaan die niet op de ingebouwde vertrouwde lijst staat, voeg dan de hostnaam toe in het veld "Additional Embed Domains" op dezelfde pagina. Deze hostnamen zijn toegestaan naast de ingebouwde providers. De overeenkomst is exact, dus voeg de volledige hostnaam toe (bijvoorbeeld player.example.com). Alles wat je niet vermeldt, blijft geblokkeerd.

Zowel het eenvoudige reactieveld als de WYSIWYG‑editor ondersteunen het plakken van een embed. In de WYSIWYG‑editor wordt de embed ingevoegd als een verwijderbaar blok.