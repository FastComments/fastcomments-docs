Agentgeheugen is tenant-gebonden, **gedeelde** sleutel-waarde-opslag die elke agent in uw tenant kan lezen en schrijven. Het bestaat zodat agenten context over runs heen kunnen meenemen.

### Waarom geheugen bestaat

LLM-context is per run. Zonder geheugen heeft een agent die een waarschuwing aan een gebruiker uitdeelt geen manier om de volgende keer dat hij dieselde gebruiker ziet die waarschuwing te kennen. Het escalatiebeleid van het platform - "waarschuwen vóór verbannen" - hangt ervan af dat de agent de eerdere waarschuwing kan vinden. Geheugen is wat dat mogelijk maakt.

### Twee soorten geheugen

- **WARNING** - wordt automatisch geschreven als onderdeel van de [`warn_user`](#tool-warn-user) flow. De agent schrijft `WARNING`-records niet zelf; ze zijn een neveneffect van het waarschuwen van een gebruiker.
- **NOTE** - geschreven door [`save_memory`](#tools-overview). Algemene context die de agent wil dat toekomstige agenten weten.

Het escalatiebeleid zoekt specifiek naar `WARNING`-records wanneer wordt beoordeeld of een ban gerechtvaardigd is.

### Tenant-gescopeerd, gedeeld tussen agenten

Alle agenten in uw tenant delen **één geheugenpool**. Een notitie die door Agent A is opgeslagen is zichtbaar in de `search_memory`-aanroepen van Agent B. Dit is opzettelijk - u wilt dat de notities van een triage-agent de beslissingen van een moderator-agent informeren.

`tenantId` wordt ingesteld door de executor vanuit de tenant van de agent zelf - nooit vanuit LLM-argumenten - dus door ontwerp zijn cross-tenant geheugenlekken onmogelijk.

### Wat er in een geheugenrecord staat

Elk geheugenitem bevat:

- **Welke agent het schreef**, en wanneer.
- **Over wie het gaat** - de gebruiker die dit geheugen beschrijft. De agent kan dit niet verzinnen; het platform vult het automatisch in op basis van wat de agent heeft geactiveerd.
- **Een verborgen alt-account signaal** - het platform registreert ook (privé) de IP fingerprint van de oorspronkelijke reactie, zodat toekomstige geheugenzoekopdrachten notities over andere accounts die vanaf hetzelfde IP posten kunnen tonen. De fingerprint wordt nooit aan de agent of de LLM getoond.
- **De notitie zelf** - tot 2000 tekens vrije tekst.
- **Tags** voor terugvinden - maximaal 10 korte tags.
- **Een soort** - ofwel een waarschuwing of een algemene notitie.
- **Een optionele comment-link** - als het geheugen aan een specifieke opmerking is gekoppeld.

### Zoekgedrag

[`search_memory`](#tools-overview) retourneert maximaal 25 records, gesorteerd op meest recent eerst, en wordt automatisch gescopeerd naar (de gebruiker die de trigger veroorzaakte) OF (andere accounts op het IP van de trigger). De resultaten zijn ook karaktergecap op 8000 totale tekens over alle geretourneerde inhoud - oudere items worden weggelaten als de limiet wordt bereikt.

De agent geeft `userId` of `targetIpHash` niet door. Beide worden door de executor ingesteld.

### Persistentie

Geheugen heeft **geen TTL**. Records blijven bestaan totdat ze expliciet worden verwijderd. WARNING-records over een gebruiker worden bewust nooit automatisch verwijderd - de escalatiegeschiedenis moet voor onbepaalde tijd vindbaar zijn, anders is de "zoeken vóór verbannen"-controle van het platform zinloos.

De drie manieren waarop geheugen wordt verwijderd:

- Een moderator verwijdert de onderliggende opmerking - elk geheugen dat aan die opmerking is gekoppeld wordt mee verwijderd.
- Een gebruiker wordt verwijderd - alle geheugenitems over die gebruiker worden in dezelfde transactie verwijderd.
- Uw tenant wordt verwijderd.

Er is momenteel geen admin-UI om individuele geheugenrecords te verwijderen.

### Geheugen bij dry-run

Dry-run agents schrijven **geen** geheugen. Dit is een bewuste keuze: de hypothetische beslissingen van een dry-run agent mogen de gedeelde geheugenpool niet vervuilen. Teruglezen via `search_memory` werkt normaal in dry-run - de agent kan echte herinneringen van live-agenten zien - maar hij kan er niets aan toevoegen.

### Geheugen bij replays

Hetzelfde als bij dry-run: replay-agents schrijven geen geheugen. Replays zijn alleen preview. Zie [Test Runs (Replays)](#test-runs-replays).

### Overzicht beperkingen

| Beperking | Waarde |
|---|---|
| Maximale lengte van geheugeninhoud | 2000 tekens |
| Maximale lengte van geheugen-tag | 64 tekens |
| Maximaal aantal geheugen-tags | 10 |
| Maximale lengte geheugenquery | 200 tekens |
| Limiet zoekresultaten geheugen | 25 vermeldingen |
| Totale karakterlimiet zoekresultaten | 8000 tekens |

### Zie ook

- [Tool: save_memory](#tools-overview) voor het schrijven.
- [Tool: search_memory](#tools-overview) voor het lezen.
- [Tool: warn_user](#tool-warn-user) - het enige hulpmiddel dat WARNING-type geheugen schrijft.
- [Tool: ban_user](#tool-ban-user) - de systeemprompt vereist dat `search_memory` wordt aangeroepen voordat dit gebeurt.

---