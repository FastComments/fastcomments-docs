**Template ID:** `gaslight_detector`

De Gaslight Detector let op commentaarbewerkingen die de geschiedenis herschrijven midden in een gesprek - het soort bewerkingen waarbij een auteur de betekenis van een eerder geplaatste reactie verandert nadat er al reacties op zijn geplaatst, waardoor de latere reacties uit hun context lijken te zijn gehaald of onjuist lijken. Wanneer de agent besluit dat een bewerking die grens overschrijdt, herstelt het de originele tekst en stuurt het de auteur een DM met uitleg.

Dit is een template met een hoger risico omdat het gebruikersinhoud wijzigt. Voer het langer uit in [dry-run](#dry-run-mode) dan je zou doen met een read-only template, en zet `edit_comment` achter een [goedkeuringsworkflow](#approval-workflow) totdat je het oordeel van het model vertrouwt op jouw verkeer.

### Triggers

- **Reactie bewerkt** (`COMMENT_EDIT`) - de agent vergelijkt de nieuwe en vorige tekst en beslist of de bewerking de reacties die al bestaan verdraait.

Zie [Trigger: Reactie bewerkt](#trigger-comment-edit) voor de volledige payload, inclusief de vorige reactietekst en het aantal reacties op het moment van bewerken.

### Toegestane tools

- [`edit_comment`](#tool-edit-comment) - gebruikt om de originele tekst te herstellen wanneer de bewerking als gaslighting wordt beoordeeld.
- [`warn_user`](#tool-warn-user) - geeft een zachte waarschuwing die de gebruiker ziet bij het volgende bezoek.
- [`send_dm`](#tools-overview) - het uitlegkanaal; de gebruiker ontvangt een direct bericht waarin wordt uitgelegd waarom zijn bewerking is teruggedraaid.

Het kan niet bannen, spam markeren, stemmen of nieuwe reacties plaatsen - het bereik is opzettelijk smal.

### Aanbevolen toevoegingen vóór livegang

- **Zet `edit_comment` achter [goedkeuring](#approval-workflow).** Het terugdraaien van een reactie is zichtbaar voor de auteur en voor iedereen die de bewerkte versie heeft gezien, dus een vals positief resultaat is gênant. Houd goedkeuringen aan totdat dry-run aantoont dat de agent consistent is.
- **Verfijn de prompt met wat als gaslighting telt op jouw site.** De standaardprompt is met opzet kort. Geef het model concrete voorbeelden - "het omdraaien van een ja/nee-bewering", "het verwijderen van een getal waarnaar reacties verwijzen", "het toevoegen van een vijandige zin nadat reacties zijn geplaatst" - en expliciete niet-voorbeelden zoals typfoutcorrecties, opschonen van opmaak of het toevoegen van bronnen.
- **Gebruik het aantal reacties uit de triggercontext.** Bewerkingen van reacties zonder reacties kunnen een gesprek niet verdraaien; de prompt moet het model opdracht geven die over te slaan.
- **Vink "Include commenter's trust factor, account age, ban history, and recent comments" aan** in [Context Options](#context-options). Het model is veel minder agressief wanneer het een langdurige account met goede bedoelingen kan zien.
- **Overweeg een korte bewerkingsgraceperiode in de prompt.** Veel bewerkingen binnen de eerste 30-60 seconden zijn typfoutcorrecties; instrueer het model om bewerkingen die zo snel plaatsvinden te negeren.

### Aanbevolen dry-run periode

Draai minimaal twee weken op echt verkeer in [dry-run](#dry-run-mode) voordat je overschakelt naar Enabled, en beoordeel elke gemarkeerde bewerking tijdens die periode. Gebruik [Test Runs (Replays)](#test-runs-replays) om de bewerkingen van de afgelopen 30 dagen tegen de agent te herhalen voordat je live gaat.