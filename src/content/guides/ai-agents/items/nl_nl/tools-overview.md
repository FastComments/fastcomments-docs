Een agent's **hulpmiddelen** zijn de acties die hij kan uitvoeren. Het agent-bewerkingsformulier heeft een **Toegestane tool-aanroepen** sectie waar je de tools aanvinkt die deze agent mag gebruiken, en een **Goedkeuringen** sectie waar je de acties aanvinkt die door een mens moeten worden goedgekeurd voordat ze effect hebben.

Er zijn drie niveaus voor elk hulpmiddel:

- **Niet toegestaan** - de agent kan het niet zien of gebruiken.
- **Toegestaan, geen goedkeuring** - de agent gebruikt het direct. Geregistreerd in de uitvoeringsgeschiedenis.
- **Toegestaan, met goedkeuring** - de aanroep van de agent wordt in de wachtrij geplaatst voor menselijke beoordeling en wordt pas uitgevoerd wanneer een mens goedkeurt.

Niet-toegestane tools zijn stil: de agent kan er niet om vragen en het platform weigert ze rechtstreeks. Tools die goedkeuring vereisen gaan altijd via de [goedkeuringsinbox](#approval-workflow).

### Auditspoor voor elke actie

Elke actie die de agent uitvoert wordt vastgelegd met een korte rechtvaardiging (1-2 zinnen die uitleggen waarom) en een confidentiescore (0.0-1.0). Beide verschijnen in de [Weergave van uitvoeringsdetails](#run-detail-view) en bij elke [goedkeuring](#approval-workflow). Het doorzoeken van geheugen is de enige read-only uitzondering: het wordt niet vastgelegd als een actie en is altijd beschikbaar ongeacht de allowlist.

### Toolreferentie

#### Reacties plaatsen

Laat de agent een reactie plaatsen als zichzelf. De reactie wordt publiekelijk getoond onder de weergavenaam van de agent. Gebruikt door greeter- en samenvattingsagents. Omkeerbaar - elke moderator kan een slechte reactie verwijderen. Meestal toegestaan zonder goedkeuring; zet er een goedkeuring voor als je community vereist dat elk publiek bericht door een mens wordt beoordeeld.

#### Stemmen op reacties

Laat de agent een op- of neerwaartse stem geven op een reactie. De stem telt mee voor het stemmen-totaal van de reactie zoals elke andere stem. De meeste communities geven er de voorkeur aan dat bots niet stemmen; niet ingeschakeld in welke startertemplate dan ook. Als je het toestaat, is stemmen omkeerbaar.

#### Een reactie vastzetten / losmaken

Laat de agent een reactie vastzetten bovenaan de pagina of een reeds vastgezette reactie losmaken. Het platform handhaaft geen regel van één pin per thread, dus een agent die vastzet moet worden geïnstrueerd om eerst de eerder vastgezette reactie los te maken. Gebruikt door de Top Comment Pinner-template. Omkeerbaar; meestal toegestaan zonder goedkeuring.

#### Een reactie vergrendelen / ontgrendelen

Laat de agent verdere antwoorden onder een reactie verhinderen, of antwoorden herstellen. De vergrendelde reactie blijft zichtbaar. Handig voor afkoeling van verhitte threads, gecombineerd met een uitgestelde ontgrendeling. Omkeerbaar maar zichtbaar voor je community; overweeg goedkeuring te vragen in communities met veel op het spel staande zaken.

#### Als spam markeren / spammarkering verwijderen

Laat de agent een reactie als spam markeren (waardoor deze voor lezers wordt verborgen en de spamclassificator wordt gevoed) of die vlag wissen. De basis-tool voor elke moderatieagent. Omkeerbaar. Overweeg sterk om in de eerste weken goedkeuring te vragen terwijl je vertrouwen in de agent opbouwt.

#### Een reactie goedkeuren / goedkeuring intrekken

Laat de agent een vastgehouden reactie aan lezers tonen, of een al zichtbare reactie verbergen. Het meest nuttig op tenants die nieuwe reacties vasthouden voor moderatorbeoordeling. Hoge inzet bij het intrekken van de goedkeuring van een zichtbare reactie - overweeg goedkeuring te vragen.

#### Een reactie als beoordeeld markeren

Een wachtrij-status tool: markeert een reactie als "een moderator (of agent) heeft dit bekeken." Verandert de zichtbaarheid niet. Lage inzet; zelden afgeschermd.

#### Een badge toekennen

Laat de agent een gebruiker een badge geven uit de badge-configuratie van je tenant. Omkeerbaar door een moderator. Zelden afgeschermd. De agent moet de badge-ID kennen, dus voeg de relevante ID's toe aan je [communityrichtlijnen](#community-guidelines) of [initiële prompt](#personality-prompt).

#### E-mail versturen

Laat de agent een platte-tekst e-mail sturen vanaf `noreply@fastcomments.com` naar een adres dat hij kiest. Spaarzaam gebruiken - e-mail is het hulpmiddel met de grootste frictie en slechte e-mails zijn moeilijk ongedaan te maken. Overweeg sterk om dit achter goedkeuring te plaatsen, en routeer goedkeurings-e-mails naar wie de inbox beheert waar de agent uiteindelijk naar zal mailen.

#### Agentgeheugen opslaan / doorzoeken

Twee gekoppelde tools die een gedeelde notitiespool lezen en schrijven over de gebruiker waarvoor een trigger afging. Geheugen wordt gedeeld tussen alle agents in je tenant, dus de aantekeningen van een triage-agent informeren de beslissingen van een moderatoragent. Zoeken is read-only en altijd beschikbaar; opslaan wordt zelden afgeschermd. Zie [Agent Memory System](#agent-memory-system) voor het volledige ontwerp.

#### Een gebruiker waarschuwen

Stuurt een privé-DM waarschuwing naar een gebruiker over een specifieke reactie, en legt atomair de waarschuwing vast in het agentgeheugen. Het escalatiebeleid van het platform is rond dit hulpmiddel opgebouwd - eerst waarschuwen, alleen verbannen als de gebruiker opnieuw de fout ingaat. Minder vaak afgeschermd dan `ban_user`, maar overweeg afscherming tijdens de eerste weken van het leven van een agent. Zie [Warn user](#tool-warn-user) voor de volledige pagina.

#### Een gebruiker verbannen

Het meest ingrijpende hulpmiddel dat een agent kan aanroepen. Verbannt een gebruiker voor een vaste duur, eventueel als shadow ban, optioneel ook het IP verbannen, optioneel ook alle reacties van de gebruiker verwijderen. De twee destructieve opties (IP, alles verwijderen) zijn afgeschermd achter extra opt-ins op het bewerkingsformulier. In de EU-regio vereisen alle verbanningen menselijke goedkeuring (zie [EU DSA Artikel 17-naleving](#eu-dsa-compliance)). Overweeg sterk om overal goedkeuring te vragen. Zie [Ban user](#tool-ban-user) voor de volledige pagina.

### Subopties van het verbanningstool

Het Ban-hulpmiddel biedt twee destructieve opties - delete-all-comments en ban-by-IP - die volledig voor het model verborgen zijn totdat je ze inschakelt via de **Ban options** sectie op het bewerkingsformulier. Zelfs als het model het parameter fantaseert, weigert het platform waarden die je niet hebt ingeschakeld. Zie [Ban user](#tool-ban-user).