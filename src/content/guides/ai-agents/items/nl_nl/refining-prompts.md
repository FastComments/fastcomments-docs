**Prompt verfijnen** is de workflow voor het bewerken van de [initial prompt](#personality-prompt) van een agent als reactie op specifieke beslissingen waar je het niet mee eens bent. Het wordt gestart vanuit de [approvals inbox](#approval-workflow).

### Wanneer te gebruiken

Wanneer je steeds hetzelfde soort goedkeuring afkeurt - "de agent wil mensen blijven verbannen voor sterk taalgebruik zonder een doelwit" - is de prompt van de agent het hefboom om dat te verhelpen. Prompt verfijnen is een begeleide manier om:

1. Een specifieke afkeuring te kiezen die de verkeerde beslissing vertegenwoordigt.
2. De prompt te bewerken met volledige context van wat de agent deed en waarom.
3. De nieuwe prompt op te slaan bij de agent.

Het resultaat is een agent die, voortaan, waarschijnlijk niet dezelfde fout meer zal maken.

### De workflow starten

Vanaf de approvals inbox op `/auth/my-account/ai-agent-approvals`:

1. Open een **rejected** approval. De route accepteert niets anders dan REJECTED - pending en execution-failed approvals komen niet in aanmerking.
2. Klik op **Prompt verfijnen**.

Je komt in de prompt-refine UI op `/auth/my-account/ai-agent-approvals/:approvalId/refine-prompt`.

### Wat de pagina toont

- **De approval** - de agent's `toolName` en `justification` voor de afgewezen beslissing (de volledige LLM-transcriptie wordt hier niet weergegeven).
- **De huidige prompt** - de opgeslagen [initial prompt](#personality-prompt) van de agent.
- **Een feedbackveld** - je typt **feedback** waarin je beschrijft wat er moet veranderen (tot 2000 tekens). De LLM genereert vervolgens de voorgestelde nieuwe prompt op basis van je feedback.
- **Unified inline diff** - een enkele inline diff tussen de huidige en de voorgestelde prompt (rood voor verwijderd, groen voor toegevoegd).

De context van de approval blijft bovenaan vastgepind zodat je tijdens het bewerken kunt blijven verwijzen naar "de zaak die ik oplos".

### Opslaan

Opslaan werkt de `initialPrompt`-veld van de agent bij. Vorige runs (en eerdere approvals) worden niet achteraf opnieuw uitgevoerd - de nieuwe prompt heeft alleen effect op toekomstige triggers. Als je wilt controleren of de nieuwe prompt het probleem oplost, voer dan een [test run / replay](#test-runs-replays) uit over de afgelopen 7 dagen en kijk of de nieuwe prompt nog steeds de afgewezen approval zou hebben geproduceerd.

### Wat de workflow niet doet

- Het bewerkt niet de **community guidelines** - dat veld heeft zijn eigen editor op het hoofdformulier voor het bewerken van de agent.
- Het bewerkt niet **triggers**, **allowed tools**, of **approval gating** - die blijven beschikbaar op het hoofdformulier voor bewerking.
- Het voert geen versiebeheer met rollback uit. De vorige prompt wordt niet opgeslagen in een aparte historietabel. Als je moet terugdraaien, kopieer dan de huidige prompt in je eigen trackingsysteem voordat je gaat bewerken.

### Waarom prompt verfijnen met replay combineren

Het bewerken van een prompt zonder het resultaat te testen is geloof gebaseerd. De aanbevolen cyclus:

1. Keur een approval af.
2. Verfijn de prompt.
3. Voer een [test run](#test-runs-replays) uit over de afgelopen 7 dagen.
4. Bekijk het tabblad **Deltas**. Heeft de nieuwe prompt de slechte beslissing verplaatst van "would do" naar "would not do"? Heeft het per ongeluk ook goede beslissingen weg verplaatst?
5. Itereer.

Drie of vier cycli van verfijnen + replay zijn meestal voldoende om een stabiele prompt voor een moderatie-agent te krijgen.

### Direct bewerken als alternatief

Je hoeft Prompt verfijnen niet te gebruiken - je kunt ook gewoon de agent bewerken op het hoofdformulier voor bewerking. Het enige voordeel van Prompt verfijnen is dat het een specifiek falend geval vastzet zodat je niet uit het oog verliest waar je voor aan het fixen bent.