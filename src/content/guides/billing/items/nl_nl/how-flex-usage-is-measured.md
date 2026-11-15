Een Flex-factuur is een lijst met regelitems, één per soort gebruik, waarbij elk item de hoeveelheid en het tarief toont, bijvoorbeeld `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. De tarieven staan op de [prijspagina](https://fastcomments.com/traffic-pricing) en op je [Factureringsgegevens](https://fastcomments.com/auth/my-account/billing-info)-pagina. In deze sectie wordt uitgelegd wat elke hoeveelheid betekent.

### Geteld Over De Maand

Deze worden elke maand gereset en geteld van de eerste tot de laatste dag van de kalendermaand (UTC):

- **Page Loads**: het aantal keren dat een commentaarwidget of feed op je pagina's is geladen. Elke keer dat de commentaarwidget laadt telt mee, ongeacht hoeveel reacties deze toont. Er wordt niet per commentaardraad of per pagina gerekend.
- **API Credits**: aanroepen naar de REST API met je API-sleutel.
- **Small Widgets Credits**: laadbeurten van de kleine widgets, zoals reactietellers en badges.
- **LLM Tokens**: tokens gebruikt door [AI Agents](/guide-ai-agents.html) en andere LLM-functies.

### Geteld Aan Het Einde Van De Maand

Dit is een momentopname die wordt gemaakt wanneer de factuur wordt aangemaakt:

- **Comments Stored**: het totale aantal reacties in je account, niet alleen die van deze maand. Het verwijderen van reacties verlaagt dit aantal, behalve dat een verwijderde reactie met antwoorden als placeholder behouden blijft zodat de antwoorden logisch blijven.
- **SSO Users**: het totale aantal SSO-gebruikers dat je site heeft aangemaakt, inclusief mensen die lang geleden één keer hebben gereageerd. Als dit het grootste onderdeel van je factuur is, vraag dan support naar facturering op basis van maandelijks actieve gebruikers, waarbij alleen gebruikers die in de maand zijn ingelogd meetellen.
- **Moderators**, **Admins**, **SSO Moderators** en **SSO Admins**: seats, behandeld in [Admin- en Moderator-seats](#admin-and-moderator-seats). De eerste admin is gratis.
- **Domains**: de sites in je account. De eerste site is gratis en elke extra site wordt per maand gefactureerd, zolang deze in het account staat. Een subdomein telt alleen mee als je het als eigen site toevoegt; één top-level domein dekt zijn subdomeinen. Zie [Meerdere Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: sub-accounts die door dit account worden beheerd, indien aanwezig, plus hun eigen gebruik als aparte regels.

### Minimum

Als de regels samen minder dan het Flex-minimum bedragen, wordt de factuur aangevuld tot het minimum. Een kleine site met één admin en één domein betaalt alleen het minimum.

### De Factuur Bekijken

De [Factureringsanalyse](https://fastcomments.com/auth/my-account/analytics/billing)-pagina toont elke regel van de volgende factuur met de cijfers van vandaag, dus er zijn geen verrassingen. De page load-, API- en small widget-tellingen kunnen tot een minuut achterlopen; de rest is live.

### Andere Valuta

Flex-gebruik wordt geprijsd in Amerikaanse dollars. Als je factuurvaluta iets anders is, wordt elke regel omgerekend tegen de wisselkoers van de dag waarop de factuur wordt aangemaakt, dus hetzelfde gebruik kan van maand tot maand iets andere bedragen kosten. Zie [Valuta's](#currencies).