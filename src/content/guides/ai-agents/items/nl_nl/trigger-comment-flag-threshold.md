Wordt geactiveerd wanneer het aantal flags van een reactie **exact** de geconfigureerde drempel bereikt.

### Vereiste configuratie

- **Flag threshold** - geheel getal >= 1. De trigger vuurt op het moment dat `flagCount === flagThreshold`. Hij vuurt niet opnieuw bij latere flags voorbij de drempel.

Als de drempel 3 is en drie gebruikers de reactie flaggen, vuurt de agent eenmaal bij de derde flag. Een vierde, vijfde of zesde flag vuurt hem **niet** opnieuw.

### Context die de agent ontvangt

- De geflagde reactie.
- Optionele thread / gebruikersgeschiedenis / pagina-context zoals geconfigureerd.
- Het aantal flags staat in het reactieblok als `Flag Count: N`.

### Opmerkingen

- De trigger vuurt alleen wanneer de reactie de drempel van onderen overschrijdt via het vlagafhandelingspad van het platform (waar `didIncrement === true`). Directe DB-schrijvingen die `flagCount` op de drempelwaarde zetten, activeren het niet; flags boven de drempel vuren het ook niet opnieuw af.
- Het bevat niet wie de reactie heeft geflagd - flags zijn anoniem voor de agent. Als je wilt weten welke gebruikers hebben geflagd, haal die informatie uit je eigen data.
- Een triggervertraging (zie [Deferred Triggers](#trigger-deferred-delay)) wordt *sterk* aanbevolen voor deze trigger - flags komen vaak in golven binnen tijdens een verhitte thread, en een kleine vertraging laat het beeld stabiliseren voordat de agent handelt.

### Veelvoorkomende toepassingen

- **Moderatiebeoordeling** - een geflagde reactie is het canonieke signaal "mensen denken dat dit mogelijk problematisch is". De [Moderator template](#template-moderator) abonneert zich standaard op deze trigger met een flagdrempel van 3.
- **Aanvulling van pre-moderatiewachtrij** - de agent voert een eerste controle uit en markeert de reactie ofwel voor moderatie (met `mark_comment_reviewed`) of escaleert dit verder.
- **Anti-brigading** - combineer deze trigger met [user history context](#context-options) en laat de agent eerdere verbanningen/duplicaat-inhoudssignalen zien voordat hij handelt.

### Aanbevelingen voor combinatie

Abonneer je op **beide** `COMMENT_ADD` en `COMMENT_FLAG_THRESHOLD` als je een moderatie-agent wilt die voor de hand liggende gevallen direct opvangt en randgevallen opnieuw beoordeelt zodra er meer flags zijn. De twee events vuren onafhankelijk van elkaar - de agent wordt twee keer uitgevoerd als beide zijn geabonneerd en beide afgaan, maar de tweede run ziet de inmiddels geflagde toestand.

---