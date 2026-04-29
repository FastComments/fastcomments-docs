De **hulpmiddelen** van een agent zijn de acties die deze kan uitvoeren. Het bewerkingsformulier van de agent heeft een **Toegestane tool-aanroepen**-sectie waar je de tools aanvinkt die deze agent mag gebruiken, en een **Goedkeuringen**-sectie waar je de acties aanvinkt die een menselijke goedkeuring moeten krijgen voordat ze van kracht worden.

There are three levels for any tool:

- **Niet toegestaan** - de agent kan het niet zien of gebruiken.
- **Toegestaan, geen goedkeuring** - de agent gebruikt het direct. Vastgelegd in de rungeschiedenis.
- **Toegestaan, met goedkeuring** - de aanroep van de agent wordt in de wachtrij gezet voor menselijke beoordeling en wordt pas uitgevoerd wanneer een mens goedkeurt.

Niet-toegestane tools zijn stil: de agent kan er niet om vragen en het platform weigert ze zonder meer. Tools die gebonden zijn aan goedkeuring gaan altijd door de [inbox voor goedkeuringen](#approval-workflow).

### Auditspoor bij elke actie

Elke actie die de agent uitvoert wordt vastgelegd met een korte rechtvaardiging (1–2 zinnen waarin wordt uitgelegd waarom) en een betrouwbaarheidscore (0.0–1.0). Beide verschijnen in de [Weergave van rundetails](#run-detail-view) en bij elke [goedkeuring](#approval-workflow). Het doorzoeken van het geheugen is de enige lees-only uitzondering: het wordt niet vastgelegd als een actie en is altijd beschikbaar ongeacht de toegestane lijst.

### Toolreferentie

#### Reacties plaatsen

Staat de agent toe een reactie namens zichzelf te plaatsen. De reactie wordt publiekelijk getoond onder de weergavenaam van de agent. Gebruikt door welkom- en samenvattingsagenten. Omkeerbaar - elke moderator kan een slechte reactie verwijderen. Meestal toegestaan zonder goedkeuring; schakel goedkeuring in als je gemeenschap wil dat elk openbaar bericht door een mens wordt gecontroleerd.

#### Reactie bewerken

Staat de agent toe de tekst van een in-scope reactie te herschrijven. De originele tekst wordt bewaard in het auditlog van de reactie. Houd dit beperkt tot smalle gevallen - het redigeren van door een gebruiker gelekte PII, of het aanpassen van een eerdere reactie van de agent zelf. Niet bedoeld om meningen te herschrijven of de toon te verzachten. **Overweeg sterk om dit achter goedkeuring te plaatsen.** Zie [Reactie bewerken](#tool-edit-comment) voor de volledige pagina.

#### Stemmen op reacties

Staat de agent toe omhoog of omlaag te stemmen op een reactie. De stem telt mee voor het totaal van de reactie zoals elke andere stem. De meeste communities geven er de voorkeur niet aan dat bots stemmen; niet ingeschakeld in enige starter-sjabloon. Als je het wel toestaat, is stemmen omkeerbaar.

#### Reactie vastpinnen / losmaken

Staat de agent toe een reactie bovenaan de pagina vast te pinnen of een reeds vastgepinde reactie los te maken. Het platform handhaaft geen regel van één pin per thread, dus een pin-agent moet geïnstrueerd worden eerst de eerder vastgepinde reactie los te maken. Gebruikt door het Top Comment Pinner-sjabloon. Omkeerbaar; meestal toegestaan zonder goedkeuring.

#### Reactie vergrendelen / ontgrendelen

Staat de agent toe verdere reacties onder een reactie te blokkeren, of reacties te herstellen. De vergrendelde reactie blijft zichtbaar. Handig voor afkoelingsperiodes in verhitte threads, te combineren met een vertraagde ontgrendeling. Omkeerbaar maar zichtbaar voor je community; overweeg goedkeuring voor communities met veel inzet.

#### Spam markeren / ontmarkeren

Staat de agent toe een reactie als spam te markeren (waardoor deze voor lezers verborgen wordt en het spamclassificatiesysteem gevoed wordt) of die vlag te verwijderen. De basis-tool voor elke moderatie-agent. Omkeerbaar. Overweeg sterk om dit in de eerste weken achter goedkeuring te plaatsen terwijl je vertrouwen in de agent opbouwt.

#### Een reactie goedkeuren / goedkeuring intrekken

Staat de agent toe een vastgehouden reactie zichtbaar te maken voor lezers, of een al zichtbare reactie te verbergen. Het meest nuttig voor tenants die nieuwe reacties vasthouden voor moderatorbeoordeling. Hoge inzet bij het intrekken van de goedkeuring van een zichtbare reactie - overweeg goedkeuring.

#### Een reactie als beoordeeld markeren

Een wachtrij-status tool: markeert een reactie als "een moderator (of agent) heeft dit bekeken." Verandert de zichtbaarheid niet. Lage inzet; zelden achter goedkeuring.

#### Een badge toekennen

Staat de agent toe een gebruiker een badge te geven uit de badge-configuratie van je tenant. Omkeerbaar door een moderator. Zelden achter goedkeuring. De agent moet het badge-ID kennen, dus neem de relevante ID's op in je [gemeenschapsrichtlijnen](#community-guidelines) of [initiële prompt](#personality-prompt).

#### E-mail verzenden

Staat de agent toe een platte-tekst e-mail te sturen vanaf `noreply@fastcomments.com` naar een adres dat hij kiest. Gebruik spaarzaam - e-mail is de tool met de meeste frictie en slechte e-mails zijn moeilijk ongedaan te maken. Overweeg sterk om dit achter goedkeuring te plaatsen, en routeer goedkeurings-e-mails naar degene die de inbox bezit waar de agent uiteindelijk naartoe zal mailen.

#### Agentgeheugen opslaan / doorzoeken

Twee gekoppelde tools die een gedeeld notitie-pool over de gebruiker lezen en schrijven waarvoor een trigger is afgevuurd. Geheugen wordt gedeeld tussen alle agents in je tenant, dus de aantekeningen van een triage-agent beïnvloeden de beslissingen van een moderator-agent. Doorzoeken is alleen-lezen en altijd beschikbaar; opslaan wordt zelden achter goedkeuring geplaatst. Zie [Agentgeheugensysteem](#agent-memory-system) voor het volledige ontwerp.

#### Een gebruiker waarschuwen

Stuurt een privé-DM waarschuwing naar een gebruiker over een specifieke reactie, en legt atomair de waarschuwing vast in agentgeheugen. Het escalatiebeleid van het platform is opgebouwd rond deze tool - eerst waarschuwen, alleen verbannen als de gebruiker opnieuw de fout maakt. Minder vaak achter goedkeuring dan `ban_user`, maar overweeg het in de eerste weken van het bestaan van een agent. Zie [Waarschuw gebruiker](#tool-warn-user) voor de volledige pagina.

#### Een gebruiker verbannen

De meest ingrijpende tool die een agent kan aanroepen. Verbannt een gebruiker met een vaste duur, optioneel als een shadow ban, optioneel ook het IP verbannend, optioneel ook alle reacties van de gebruiker verwijderend. De twee destructieve opties (IP, delete-all) zijn achter extra opt-ins op het bewerkingsformulier verborgen. In de EU-regio vereisen alle bans menselijke goedkeuring (zie [EU DSA Artikel 17-naleving](#eu-dsa-compliance)). Overweeg sterk om dit overal achter goedkeuring te plaatsen. Zie [Een gebruiker verbannen](#tool-ban-user) voor de volledige pagina.

### Subopties van de Ban-tool

De Ban-tool biedt twee destructieve opties - delete-all-comments en ban-by-IP - die volledig verborgen zijn voor het model totdat je ze via de **Ban-opties**-sectie op het bewerkingsformulier inschakelt. Zelfs als het model de parameter hallucineert, weigert het platform waarden die je niet hebt ingeschakeld. Zie [Een gebruiker verbannen](#tool-ban-user).