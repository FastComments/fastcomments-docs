---
Omdat e-mailsjablonen variabelen en logica ondersteunen, is het mogelijk sjablonen te maken
die niet gerenderd worden, of soms niet gerenderd worden.

Dit kan erg frustrerend zijn om te diagnosticeren en te debuggen, vooral als het een intermitterend probleem is, of
als het alleen voorkomt wanneer de gegevens er op een bepaalde manier uitzien.

Om te helpen, FastComments Email Templates heeft een paar functies:

1. Als het sjabloon niet kan worden voorvertoond, kan het niet worden opgeslagen. Er wordt een foutmelding weergegeven.
2. Renderfouten van sjablonen worden bijgehouden en gerapporteerd in de beheer-UI.

De tweede bullet beschrijft renderfouten die in productie optreden. Dat wil zeggen, je maakt een sjabloon dat in de voorbeeldweergave
goed werkt - maar later om een of andere reden faalt. Bijvoorbeeld, als we dit in ons sjabloon hebben:

    <% if (comment.commenterEmail.includes('test') { %>

Dit kan soms falen als anoniem reageren is ingeschakeld, omdat e-mail niet altijd
beschikbaar zal zijn. Dus hoe komen we daar achter?

Het antwoord is dat fouten op twee plaatsen worden weergegeven. Ten eerste toont de sjabloonlijst zelf
een aantal renderfouten bij elk sjabloon.

Vervolgens, wanneer je een sjabloon bekijkt, zien we per fout een telling van het aantal keren dat het sjabloon
niet kon worden gerenderd.

Een resetknop bevindt zich naast elke fout en de bijbehorende telling, zodat we de teller
kunnen resetten nadat we het probleem hebben opgelost.

---