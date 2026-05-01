De **tools** van een agent zijn de acties die deze kan uitvoeren. Het bewerkingsformulier voor de agent heeft een sectie **Toegestane tool-oproepen** waar je de tools aanvinkt die deze agent mag gebruiken, en een sectie **Goedkeuringen** waar je de acties aanvinkt die door een mens goedgekeurd moeten worden voordat ze van kracht worden.

Er zijn drie niveaus voor elke tool:

- **Niet toegestaan** - de agent kan het niet zien of gebruiken.
- **Toegestaan, geen goedkeuring** - de agent gebruikt het direct. Geregistreerd in de rungeschiedenis.
- **Toegestaan, met goedkeuring** - de oproep van de agent wordt in de wachtrij gezet voor menselijke beoordeling en wordt alleen uitgevoerd wanneer een mens deze goedkeurt.

Niet-toegestane tools zijn stil: de agent kan er niet om vragen en het platform weigert ze rechtstreeks. Tools die afhankelijk zijn van goedkeuring gaan altijd via de [inbox voor goedkeuringen](#approval-workflow).

### Auditspoor bij elke actie

Elke actie die de agent uitvoert wordt vastgelegd met een korte rechtvaardiging (1–2 zinnen die uitleggen waarom) en een confidentiescore (0.0–1.0). Beide verschijnen in de [Weergave rundetails](#run-detail-view) en bij elke [goedkeuring](#approval-workflow). Het doorzoeken van het geheugen is de ene read-only uitzondering: het wordt niet geregistreerd als een actie en is altijd beschikbaar ongeacht de allowlist.

### Toolreferentie

#### Reacties plaatsen

Laat de agent een reactie plaatsen namens zichzelf. De reactie wordt openbaar weergegeven onder de weergavenaam van de agent. Gebruikt door welkomst- en samenvattende agenten. Omkeerbaar - elke moderator kan een ongepaste reactie verwijderen. Zet dit achter een goedkeuring als jouw community vereist dat elk publiek-facing bericht door een mens wordt beoordeeld.

#### Een reactie bewerken

Laat de agent de tekst van een in‑scope reactie herschrijven. De originele tekst wordt bewaard in het auditlog van de reactie. Beperk dit tot beperkte gevallen - het redigeren van PII die een gebruiker lekte, of het aanpassen van een eerdere reactie van de agent zelf. Niet bedoeld om meningen te herschrijven of de toon te verzachten. Zie [Reactie bewerken](#tool-edit-comment) voor de volledige pagina.

#### Stemmen op reacties

Laat de agent stemmen voor of tegen een reactie. De stem telt mee in het totaal aantal stemmen zoals elke andere stem. De meeste communities geven er de voorkeur aan dat bots niet stemmen; niet ingeschakeld in een starter‑template. Als je het wel toestaat, is stemmen omkeerbaar.

#### Reactie vastzetten / losmaken

Laat de agent een reactie bovenaan de pagina vastzetten of een reeds vastgezette reactie losmaken. Het platform handhaaft geen regel van één pin per thread, dus een pin‑agent moet geïnstrueerd worden om eerst de vorige vastgezette reactie los te maken. Om te ontdekken wat al vastgezet is op dezelfde pagina, kan de agent de read-only tool `get_pinned_comments` aanroepen (zie hieronder). Gebruikt door het Top Comment Pinner-sjabloon.

#### Reactie vergrendelen / ontgrendelen

Laat de agent verdere antwoorden onder een reactie voorkomen, of reacties herstellen. De vergrendelde reactie blijft zichtbaar. Handig voor afkoeling van verhitte discussies, gecombineerd met een uitgestelde ontgrendeling. Om te ontdekken wat momenteel vergrendeld is op dezelfde pagina kan de agent de read-only tool `get_locked_comments` aanroepen (zie hieronder).

#### Spam markeren / de-markeren

Laat de agent een reactie als spam markeren (waardoor deze voor lezers verborgen wordt en wordt gevoed naar de spamclassifier) of die vlag verwijderen. De basistool voor elke moderatieagent. Omkeerbaar.

#### Goedkeuren / goedkeuring intrekken van een reactie

Laat de agent een vastgehouden reactie zichtbaar maken voor lezers, of een al zichtbare reactie verbergen. Het meest nuttig op tenants die nieuwe reacties vasthouden voor moderatorbeoordeling.

#### Een reactie als beoordeeld markeren

Een queue‑status tool: markeert een reactie als "een moderator (of agent) heeft hiernaar gekeken." Wijzigt de zichtbaarheid niet. Lage inzet; zelden achter goedkeuring geplaatst.

#### Een badge toekennen

Laat de agent een gebruiker een badge geven die je voor jouw tenant hebt geconfigureerd. Omkeerbaar door een moderator. Wanneer deze tool is ingeschakeld kan de agent de badges van jouw tenant zien en zelf de juiste kiezen, dus je hoeft geen badge‑identificatoren in je communityrichtlijnen of initiële prompt te plakken. Om te sturen welke badge voor welk gedrag wordt toegekend, verwijs naar de badges met hun **Weergave‑label** in de prompt.

#### E-mail verzenden

Laat de agent een platte‑tekst e-mail sturen naar de auteur van een reactie binnen de scope van de trigger. De agent ziet nooit het e-mailadres van de ontvanger - hij kiest een reactie en het platform bezorgt naar het adres dat die poster achterliet bij het plaatsen. Het afzenderadres is de branded afzender van jouw tenant (met DKIM) wanneer het domein van de reactie overeenkomt met een geconfigureerd domein, anders de platform‑standaard. Gebruik spaarzaam - e-mail is de tool met de hoogste frictie en slechte e-mails zijn moeilijk ongedaan te maken.

#### Agentgeheugen opslaan / doorzoeken

Twee gekoppelde tools die een gedeeld notitiepool lezen en schrijven over de gebruiker waarvoor een trigger afging. Geheugen wordt gedeeld over alle agents in jouw tenant, zodat de notities van een triage‑agent de beslissingen van een moderator‑agent informeren. Zoeken is read-only en altijd beschikbaar; opslaan wordt zelden achter een goedkeuring geplaatst. Zie [Agentgeheugensysteem](#agent-memory-system) voor het volledige ontwerp.

#### Pinned comments ophalen / Locked comments ophalen

Twee read-only discovery tools die de vastgezette (of vergrendelde) reacties op dezelfde pagina (`urlId`) waar de trigger op afging, opsommen. Ze nemen geen argumenten aan - de pagina wordt gelezen vanuit de triggercontext, dus de agent kan niet naar andere pagina's pivoteren. Gebruik ze wanneer een agent op een reactie moet handelen die al vastgezet of vergrendeld is - doorgaans de eerste oproep vóór `unpin_comment` of `unlock_comment`, of vóór het vastzetten van een nieuwe reactie zodat de bestaande eerst kan worden losgemaakt.

Elke tool wordt afzonderlijk geregeld in **Toegestane tool-oproepen** (de beheerder vinkt `List pinned comments on the current page` of `List locked comments on the current page` aan). Ze kunnen niet achter een goedkeuring worden geplaatst - read-only tools hebben geen bijwerking om goed te keuren. Het aanroepen van deze tools wordt niet als actie vastgelegd in de rungeschiedenis; alleen de resulterende `unpin_comment` / `unlock_comment` / `pin_comment` oproep (indien aanwezig) verschijnt. De lijst is per oproep beperkt tot de meest recente 20 overeenkomsten.

Belangrijk om te begrijpen: wanneer een van deze tools een commentId retourneert, wordt die commentId toegevoegd aan de per-run scope van de agent, zodat de daaropvolgende `unpin_comment` / `unlock_comment` oproep valideert tegen de tool‑target veiligheidscontrole van het platform. Zonder eerst de discovery‑tool aan te roepen, kan de agent niet handelen op reacties die niet al in de onmiddellijke scope van de trigger zitten. Dus een unpin‑achtige agent krijgt typisch beide tools ingeschakeld (bijv. `get_pinned_comments` plus `unpin_comment`).

#### Een gebruiker waarschuwen

Stuurt een privé DM‑waarschuwing naar een gebruiker over een specifieke reactie, en legt de waarschuwing atomair vast in het agentgeheugen. Het escalatiebeleid van het platform is rond deze tool opgebouwd - eerst waarschuwen, alleen verbannen als de gebruiker opnieuw de fout in gaat. Zie [Gebruiker waarschuwen](#tool-warn-user) voor de volledige pagina.

#### Een gebruiker verbannen

De meest ingrijpende tool die een agent kan aanroepen. Verbannt een gebruiker voor een vaste duur, optioneel als shadow ban, optioneel ook het IP verbannend, optioneel ook door alle reacties van de gebruiker te verwijderen. De twee destructieve opties (IP, delete-all-comments) worden achter extra opt‑ins op het bewerkingsformulier gezet. In de EU‑regio vereisen alle bans menselijke goedkeuring (zie [EU DSA Artikel 17-naleving](#eu-dsa-compliance)). Zie [Gebruiker verbannen](#tool-ban-user) voor de volledige pagina.

### Subopties van de Ban-tool

De Ban‑tool biedt twee destructieve opties - delete-all-comments en ban-by-IP - die volledig voor het model verborgen zijn totdat je ze inschakelt via de sectie **Ban-opties** op het bewerkingsformulier. Zelfs als het model een parameter zou hallucineren, weigert het platform waarden waarvoor je niet hebt gekozen. Zie [Gebruiker verbannen](#tool-ban-user).