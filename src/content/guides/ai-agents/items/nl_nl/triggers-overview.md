Een **trigger** is een gebeurtenis die een agent activeert. Elke agent kan één of meer triggers gedefinieerd hebben.

### De volledige lijst

| Trigger | Wanneer het wordt geactiveerd |
|---|---|
| [Reactie toegevoegd](#trigger-comment-add) | Er wordt een nieuwe reactie geplaatst. |
| [Reactie bewerkt](#trigger-comment-edit) | Een reactie is bewerkt. De vorige tekst wordt opgenomen in de context van de agent. |
| [Reactie verwijderd](#trigger-comment-delete) | Een reactie wordt verwijderd. |
| [Reactie vastgezet](#trigger-comment-pin) | Een reactie wordt vastgezet (door wie dan ook, inclusief een moderator of een andere agent). |
| [Reactie losgemaakt](#trigger-comment-unpin) | Een reactie wordt losgemaakt. |
| [Reactie vergrendeld](#trigger-comment-lock) | Een reactie wordt vergrendeld (geen verdere antwoorden toegestaan). |
| [Reactie ontgrendeld](#trigger-comment-unlock) | Een reactie wordt ontgrendeld. |
| [Reactie overschrijdt stemdrempel](#trigger-comment-vote-threshold) | De netto stemmen van een reactie bereiken de geconfigureerde drempel. |
| [Reactie overschrijdt vlagdrempel](#trigger-comment-flag-threshold) | Het aantal flags van een reactie bereikt exact de geconfigureerde drempel. |
| [Gebruiker plaatst eerste reactie](#trigger-new-user-first-comment) | Een gebruiker plaatst voor het eerst een reactie op deze site. |
| [Reactie automatisch als spam gemarkeerd](#trigger-comment-auto-spammed) | Een reactie wordt automatisch als spam gemarkeerd door de spam-engine. |
| [Moderator beoordeelt reactie](#trigger-moderator-reviewed) | Een moderator markeert een reactie als beoordeeld. |
| [Moderator keurt reactie goed](#trigger-moderator-approved) | Een moderator keurt een reactie goed. |
| [Moderator markeert spam](#trigger-moderator-spammed) | Een moderator markeert een reactie als spam. |
| [Moderator kent badge toe](#trigger-moderator-awarded-badge) | Een moderator kent een badge toe aan een gebruiker. |

### Meerdere triggers per agent

Een agent kan zich abonneren op elke combinatie van triggers - het [Moderator-sjabloon](#template-moderator) abonneert zich bijvoorbeeld op zowel `COMMENT_ADD` als `COMMENT_FLAG_THRESHOLD`. Elke gebeurtenis activeert de agent één keer met de context van die gebeurtenis.

### Wat voorkomt dat een agent wordt geactiveerd

Een geabonneerde triggerevenement activeert de agent **niet** als een van de volgende situaties van toepassing is:

- De [status](#status-states) van de agent is **Uitgeschakeld**.
- De [URL- of locale-scope](#scope-url-locale) van de agent komt niet overeen met de triggerende reactie.
- Het [dagelijkse, maandelijkse of rate-limit-budget](#budgets-overview) van de agent is uitgeput - de trigger wordt als **verworpen** geregistreerd met een reden. Zie [Redenen voor verwerping](#drop-reasons).
- De gelijktijdigheid voor deze agent is verzadigd (per-agent begrensd).
- De tenant van de agent heeft ongeldige facturering.
- De triggerende actie is zelf uitgevoerd door een bot of een andere agent (ter voorkoming van lussen).
- De trigger was voor een reactie die al door deze agent is verwerkt binnen het deduplicatievenster.

Wanneer een geabonneerde trigger succesvol wordt geactiveerd, toont de [Run History](#run-history) van de agent een regel met status **Gestart** die overgaat in **Succes** of **Fout** wanneer de run is voltooid.

### Stem- en vlagdrempels

Twee triggers - **Comment Crosses Vote Threshold** en **Comment Crosses Flag Threshold** - vereisen een numerieke drempel op het bewerkingsformulier. De trigger vuurt op het moment dat het aantal de geconfigureerde waarde overschrijdt (concreet vuurt de vlagdrempel-trigger wanneer `flagCount === flagThreshold`, dus het kiezen van 1 betekent "vuurt bij de eerste melding", en het kiezen van 5 betekent "vuurt wanneer de vijfde melding arriveert").

### Uitgestelde triggers

Elke trigger kan worden uitgesteld zodat de agent later draait, bijvoorbeeld nadat stemmen/meldingen/antwoorden de tijd hebben gehad om te stabiliseren. Zie [Uitgestelde triggers](#trigger-deferred-delay).

### Voorkomen van lussen

Om oneindige lussen te voorkomen dragen reacties **geschreven door een agent** een `botId`. Nieuw-reactie-triggers negeren reacties met een `botId`.

Het netto-effect: agents kunnen reageren op *menselijke* acties in uw tenant, maar door agents gegenereerde acties activeren nooit agent-triggers. Dit geldt voor alle triggertypen.

### REPLAY: de interne trigger

Er is ook een interne `REPLAY` trigger-type die wordt gebruikt door de functie [Test Runs (Replays)](#test-runs-replays). Je kunt deze niet selecteren op het bewerkingsformulier - het bestaat zodat replay-runs duidelijk worden getagd in het runoverzicht en uitgesloten zijn van weergaven van live-runs.