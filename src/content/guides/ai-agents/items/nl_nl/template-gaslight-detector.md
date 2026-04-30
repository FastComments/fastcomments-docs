**Template ID:** `gaslight_detector`

De Gaslight Detector houdt wijzigingen in opmerkingen in de gaten die het verloop van een gesprek herschrijven - het soort waarbij een auteur de betekenis van een eerdere opmerking verandert nadat er al reacties zijn geschreven, waardoor achterliggende reacties uit de context of onjuist lijken. Wanneer de agent besluit dat een bewerking die grens overschrijdt, herstelt hij de originele tekst en stuurt hij de auteur een DM om het uit te leggen.

Dit is een risicoverhogende template omdat deze gebruikerscontent wijzigt. Voer het uit in [dry-run](#dry-run-mode) langer dan je dat bij een read-only template zou doen, en zet `edit_comment` achter [approval](#approval-workflow) totdat je het oordeel van het model op jouw verkeer vertrouwt.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - de agent vergelijkt de nieuwe tekst met de vorige tekst en besluit of de bewerking reeds bestaande reacties vertekent.

Zie [Trigger: Comment Edited](#trigger-comment-edit) voor de volledige payload, inclusief de vorige opmerkingstekst en het aantal reacties op het moment van bewerken.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - wordt gebruikt om de originele tekst te herstellen wanneer de bewerking wordt beoordeeld als gaslighting.
- [`warn_user`](#tool-warn-user) - geeft een zachte waarschuwing die de gebruiker bij zijn volgende bezoek ziet.
- [`send_dm`](#tools-overview) - het uitlegkanaal; de gebruiker ontvangt een direct bericht met de reden waarom zijn bewerking is teruggedraaid.

Hij kan niet verbannen, als spam markeren, stemmen of nieuwe opmerkingen plaatsen - de scope is opzettelijk beperkt.

### Recommended additions before going live

- **Gate `edit_comment` behind [approval](#approval-workflow).** Het terugdraaien van een opmerking is zichtbaar voor de auteur en voor iedereen die de bewerkte versie heeft gezien, dus een false positive is beschamend. Houd goedkeuringen aan totdat dry-run laat zien dat de agent consequent handelt.
- **Tighten the prompt with what counts as gaslighting on your site.** De standaardprompt is opzettelijk kort. Geef het model concrete voorbeelden - "flipping a yes/no claim", "deleting a number that replies cite", "adding a hostile sentence after replies were posted" - en expliciete niet-voorbeelden zoals typefouten corrigeren, opmaak opschonen of het toevoegen van bronnen.
- **Use the reply count from the trigger context.** Bewerkingen van opmerkingen zonder reacties kunnen een gesprek niet vervormen; de prompt moet het model instrueren die over te slaan.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** in [Context Options](#context-options). Het model is veel minder agressief wanneer het een lang bestaande account met goede bedoelingen kan zien.
- **Consider a short edit-grace window in the prompt.** Veel bewerkingen binnen de eerste 30–60 seconden zijn typefouten; geef het model de opdracht bewerkingen die snel daarna plaatsvinden te negeren.

### Recommended dry-run window

Draai het minimaal twee weken met echt verkeer in [dry-run](#dry-run-mode) voordat je overschakelt naar Enabled, en beoordeel elke gemarkeerde bewerking tijdens die periode. Gebruik [Test Runs (Replays)](#test-runs-replays) om de laatste 30 dagen aan bewerkingen tegen de agent te replayen voordat je live gaat.

---