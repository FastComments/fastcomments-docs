De "tools" van een agent zijn de acties die deze kan uitvoeren. Het bewerkingsformulier voor een agent heeft een sectie **Allowed tool calls** waar je de tools aanvinkt die deze agent mag gebruiken, en een sectie **Approvals** waar je de acties aanvinkt die door een mens goedgekeurd moeten worden voordat ze effect hebben.

Er zijn drie niveaus voor elke tool:

- **Disallowed** - de agent kan deze niet zien of gebruiken.
- **Allowed, no approval** - de agent gebruikt deze direct. Geregistreerd in de uitvoeringsgeschiedenis.
- **Allowed, with approval** - de oproep van de agent wordt in de wachtrij geplaatst voor menselijke beoordeling en wordt alleen uitgevoerd als een mens goedkeurt.

Niet-toegestane tools zijn stil: de agent kan er niet om vragen en het platform wijst ze resoluut af. Tools die goedkeuring vereisen gaan altijd via de [inbox voor goedkeuringen](#approval-workflow).

### Audit trail on every action

Elke actie die de agent onderneemt wordt vastgelegd met een korte rechtvaardiging (1–2 zinnen die uitleggen waarom) en een vertrouwensscore (0.0–1.0). Beide verschijnen in de [Weergave van rundetails](#run-detail-view) en bij elke [goedkeuring](#approval-workflow). Het doorzoeken van geheugen is de enige lees-only uitzondering: het wordt niet vastgelegd als een actie en is altijd beschikbaar ongeacht de allowlist.

### Tool reference

#### Posting comments

Staat de agent toe een opmerking als zichzelf te plaatsen. De opmerking wordt publiekelijk getoond onder de weergavenaam van de agent. Wordt gebruikt door verwelkomings- en samenvattingsagents. Omkeerbaar - elke moderator kan een slechte opmerking verwijderen. Meestal toegestaan zonder goedkeuring; schakel goedkeuring in als jouw community wil dat elk publiek bericht door een mens wordt beoordeeld.

#### Editing a comment

Staat de agent toe de tekst van een opmerking binnen scope te herschrijven. De originele tekst wordt bewaard in het auditlog van de opmerking. Reserveer voor beperkte gevallen - het redigeren van PII die een gebruiker heeft gelekt, of het aanpassen van de eerdere reactie van de agent zelf. Niet bedoeld voor het herschrijven van meningen of het verzachten van toon. **Overweeg sterk om dit achter goedkeuring te plaatsen.** Zie [Opmerking bewerken](#tool-edit-comment) voor de volledige pagina.

#### Voting on comments

Staat de agent toe een opmerking omhoog of omlaag te stemmen. De stem telt mee voor het stemtotaal van de opmerking zoals elke andere stem. De meeste communities geven er de voorkeur aan dat bots niet stemmen; niet ingeschakeld in een startersjabloon. Als je het toestaat, is stemmen omkeerbaar.

#### Pin / unpin a comment

Staat de agent toe een opmerking bovenaan de pagina vast te zetten of een reeds vastgezette opmerking los te maken. Het platform handhaaft geen regel van één pin per thread, dus een pinning-agent moet geïnstrueerd worden eerst de eerder vastgezette opmerking los te maken. Wordt gebruikt door het Top Comment Pinner-sjabloon. Omkeerbaar; meestal toegestaan zonder goedkeuring.

#### Lock / unlock a comment

Staat de agent toe verdere antwoorden onder een opmerking te voorkomen of antwoorden te herstellen. De vergrendelde opmerking blijft zichtbaar. Handig voor afkoelperioden bij verhitte discussies, te combineren met een uitgestelde ontgrendeling. Omkeerbaar maar zichtbaar voor je community; overweeg goedkeuring op hoog-risico communities.

#### Mark / unmark spam

Staat de agent toe een opmerking als spam te markeren (zodat deze voor lezers wordt verborgen en de spamclassifier wordt gevoed) of die vlag te verwijderen. De belangrijkste tool voor elke moderatie-agent. Omkeerbaar. Overweeg sterk om dit in de eerste weken achter goedkeuring te plaatsen terwijl je vertrouwen in de agent opbouwt.

#### Approve / un-approve a comment

Staat de agent toe een vastgehouden opmerking aan lezers te tonen, of een reeds zichtbare opmerking te verbergen. Meest nuttig voor tenants die nieuwe opmerkingen vasthouden voor moderatoreview. Hoog risico bij het de-publiceren van een zichtbare opmerking - overweeg overal goedkeuring.

#### Mark a comment reviewed

Een wachtrij-status-tool: markeert een opmerking als "een moderator (of agent) heeft dit bekeken." Verandert de zichtbaarheid niet. Weinig risicovol; zelden afgezet achter goedkeuring.

#### Award a badge

Staat de agent toe een gebruiker een badge te geven die je voor je tenant hebt geconfigureerd. Ongedaan te maken door een moderator. Zelden afgezet. Wanneer deze tool is ingeschakeld, kan de agent de badges van je tenant zien en zelf de juiste kiezen, dus je hoeft geen badge-identificatoren in je communityrichtlijnen of initiële prompt te plakken. Als je wilt sturen welke badge voor welk gedrag wordt toegekend, verwijs dan naar de badges met hun **Display Label** in de prompt.

#### Send email

Staat de agent toe een platte-tekst e-mail te sturen naar de auteur van een opmerking binnen de scope van de trigger. De agent ziet nooit het e-mailadres van de ontvanger - hij kiest een opmerking en het platform levert af naar het adres dat die commenter heeft achtergelaten bij het plaatsen. Het from-adres is de merkafzender van je tenant (met DKIM) wanneer het domein van de opmerking overeenkomt met een geconfigureerd domein, anders het platformstandaardadres. Gebruik spaarzaam - e-mail is de tool met de hoogste frictie en slechte e-mails zijn moeilijk ongedaan te maken. Overweeg sterk om dit achter goedkeuring te plaatsen en routeer goedkeurings-e-mails naar degene die eigenaar is van de inbox die de agent uiteindelijk zal e-mailen.

#### Save / search agent memory

Twee gekoppelde tools die een gedeelde notitiepool lezen en schrijven over de gebruiker waarvoor een trigger werd geactiveerd. Geheugen wordt gedeeld tussen alle agents in je tenant, dus de notities van een triage-agent informeren de beslissingen van een moderator-agent. Search is read-only en altijd beschikbaar; saving wordt zelden afgezet achter goedkeuring. Zie [Systeem voor agentgeheugen](#agent-memory-system) voor het volledige ontwerp.

#### Warn a user

Stuurt een private DM-waarschuwing naar een gebruiker over een specifieke opmerking, en legt atomair de waarschuwing vast in het agentgeheugen. Het escalatiebeleid van het platform is rond deze tool opgebouwd - eerst waarschuwen, pas verbannen als de gebruiker opnieuw in de fout gaat. Minder vaak afgezet dan `ban_user`, maar overweeg het af te zetten in de eerste weken van het leven van een agent. Zie [Gebruiker waarschuwen](#tool-warn-user) voor de volledige pagina.

#### Ban a user

De meest ingrijpende tool die een agent kan aanroepen. Verban een gebruiker voor een vaste duur, eventueel als een shadow ban, optioneel ook het IP verbannen, optioneel ook alle opmerkingen van de gebruiker verwijderen. De twee destructieve opties (IP, delete-all-comments) zijn achter extra opt-ins op het bewerkingsformulier verborgen. In de EU-regio vereisen alle bans menselijke goedkeuring (zie [Naleving EU DSA Artikel 17](#eu-dsa-compliance)). Overweeg sterk om dit overal achter goedkeuring te plaatsen. Zie [Gebruiker verbannen](#tool-ban-user) voor de volledige pagina.

### Ban-tool sub-options

De Ban-tool biedt twee destructieve opties - delete-all-comments en ban-by-IP - die volledig voor het model verborgen zijn totdat je ze via de sectie **Ban options** op het bewerkingsformulier inschakelt. Zelfs als het model de parameter hallicineert, weigert het platform waarden die je niet hebt ingeschakeld. Zie [Gebruiker verbannen](#tool-ban-user).