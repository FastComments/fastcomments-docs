Standaard vertaalt FastComments de eigen interface van de widget (knoppen, labels, foutmeldingen) naar de taal van de kijker, maar reacties worden precies getoond zoals ze zijn geschreven.

Comment Translation voegt vertaling van de reacties zelf toe, zodat een community die in meer dan één taal schrijft elke reactie in de eigen taal kan lezen.

Er zijn twee modi:

- **Automatisch**: reacties die in een andere taal dan de taal van de widget zijn geschreven, worden vertaald getoond. Onder de reactie verschijnt een regel "Vertaald uit het Spaans" met een **Show original**-link die wisselt tussen de vertaling en de onvertaalde tekst.
- **Handmatig**: elke reactie krijgt een **Translate**-link. Door erop te klikken wordt die ene reactie vertaald naar de taal van de kijker, met daarna dezelfde **Show original**-schakelaar. Reacties die al in de taal van de kijker zijn, zeggen dat in plaats daarvan.

De doeltaal is de taal van de widget: de instelling [Locale / Language](#locale) op dezelfde aanpassingsregel als die is ingesteld, anders de browsertaal van de bezoeker. Er wordt niets van de bezoeker vereist, geen account, extensie of extra software, en het werkt hetzelfde in de ingesloten widget en op standalone reactiepagina's.

Dit wordt per <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">aanpassingsregel</a> ingeschakeld, dus het kan van toepassing zijn op je hele site, één domein of één pagina. Het staat standaard uit.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Comment Translation dropdown op de widget-aanpassingspagina met de opties Off, Automatic en Manual'; title='Comment Translation' app-screenshot-end]

Vertaling is een server-side instelling. Het doorgeven van `commentTranslation` in de JavaScript-configuratie van de widget schakelt het niet in; de regel doet dat.

Elke reactie wordt maximaal één keer per doeltaal vertaald en het resultaat wordt bij de reactie opgeslagen, zodat een reactie die door duizend Spaanstaligen wordt gelezen maar één keer wordt vertaald. Als de auteur of een moderator de reactie bewerkt, wordt de opgeslagen vertaling verwijderd en wordt de reactie bij de volgende weergave opnieuw vertaald. Vertaalde tekst gaat door dezelfde sanitization- en linkregels als de originele reactie. Reacties die te lang zijn, of waarvan de taal niet kan worden gedetecteerd, worden onvertaald getoond.

Gebruik wordt gefactureerd als LLM-tokens tegen hetzelfde tarief als [AI Agents](/guide-ai-agents.html#plans-and-eligibility) en verschijnt op dezelfde plek in je factureringsgegevens.