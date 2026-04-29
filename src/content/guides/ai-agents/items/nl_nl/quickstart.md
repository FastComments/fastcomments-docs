Dit is het vijf-minutenpad van "we hebben AI-agents" naar "een agent reageert op live verkeer, afgeschermd door goedkeuringen." Als je de lange versie wilt, linkt elke stap naar de pagina die het diepgaand behandelt.

### 1. Open de AI Agents-pagina

Ga naar [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) in je account. De eerste keer dat je hier terechtkomt zie je of:

- Een leeg-scherm met een **Blader door sjablonen**- en **Vanaf nul beginnen**-knop (je hebt agents beschikbaar om te maken), of
- Een upsellpagina als je plan geen agents omvat - zie [Plannen en geschiktheid](#plans-and-eligibility).

### 2. Kies een start-sjabloon

Klik **Blader door sjablonen**. Kies een van:

- [Moderator](#template-moderator) - beoordeelt gemarkeerde of nieuwe opmerkingen, waarschuwt eerstegangsgebruikers, escaleert naar verbanning alleen na een waarschuwing.
- [Welkomstgroeter](#template-welcome-greeter) - reageert op eerste keren commentatoren.
- [Topreactie-vastzetter](#template-top-comment-pinner) - zet substantiële reacties vast zodra ze een stemdrempel overschrijden.
- [Draad-samenvatter](#template-thread-summarizer) - plaatst een neutrale samenvatting bij lange discussies.

Elk sjabloon opent een vooraf ingevuld bewerkingsformulier met **Status: Testmodus** al geselecteerd.

### 3. Beoordeel en opslaan

Op het bewerkingsformulier vul je minimaal in:

- **Interne naam.** Een korte identificatie gebruikt in administratieborden.
- **Weergavenaam.** Wat openbaar verschijnt wanneer de agent een opmerking plaatst.
- **Initiële prompt.** Bewerk de prompt van het sjabloon zodat deze jouw toon en specifieke regels weerspiegelt.
- **Goedkeuringen.** Vink de acties aan die een menselijke beoordeling moeten vereisen voordat ze van kracht worden. We raden ten minste `ban_user` aan voor elk agent die moderatie-achtig werk doet. Zie [Approval Workflow](#approval-workflow).

Klik **Agent opslaan**.

### 4. Bekijk het in testmodus

De agent is nu live in **Testmodus**. Hij zal zijn triggers ontvangen, het model aanroepen en acties registreren op de [Uitvoeringsgeschiedenis](#run-history)-pagina - met de **Testmodus**-badge op elke regel - maar hij voert geen echte acties uit. Bezoek een paar uitvoeringsdetails (zie [Uitvoeringsdetailweergave](#run-detail-view)) en kijk naar:

- De acties die de agent koos.
- De onderbouwing en het vertrouwen bij elke actie.
- Het volledige LLM-transcript.

Als de agent beslissingen neemt waar je het niet mee eens bent, bewerk dan de initiële prompt of vink meer goedkeuringen aan.

### 5. Voer een test uit tegen eerdere opmerkingen

Vanaf de agents-lijstpagina klik je **Testuitvoering** op de regel van de agent. Het formulier heeft één numeriek invoerveld **Dagen** (1 tot 90). Samplegrootte en het harde maximum aan beoordeelde opmerkingen worden informatief weergegeven - ze worden server-side berekend, niet door de gebruiker ingesteld. De replay draait tegen historische opmerkingen zonder echte acties uit te voeren en rapporteert wat de agent **zou** hebben gedaan versus wat er daadwerkelijk gebeurde (werd de opmerking later goedgekeurd, gemarkeerd als spam, verwijderd, enz.). Zie [Test Runs (Replays)](#test-runs-replays).

### 6. Zet op 'Ingeschakeld'

Wanneer je tevreden bent met de testmodus- en replay-uitvoer, bewerk je de agent en wijzig je **Status** naar **Ingeschakeld**. Vanaf dat moment worden echte acties uitgevoerd. De Uitvoeringsgeschiedenis-pagina toont nu live uitvoeringen zonder de testmodus-badge, en elke actie die je voor goedkeuring hebt gemarkeerd verschijnt in de [goedkeuringsinbox](#approval-workflow).

### Wat nu

- Stel [Budgetten](#budgets-overview) en [Budgetwaarschuwingen](#budget-alerts) in.
- Configureer [Webhooks](#webhooks-overview) als je wilt dat externe systemen reageren op agentgebeurtenissen.
- Voeg [Richtlijnen voor de community](#community-guidelines) toe om agentbeslissingen in lijn te houden met je schriftelijke beleid.