#### "Registratietoken niet gevonden, verlopen of al gebruikt"

Het token in uw registratie-URL is 30 minuten geldig en kan slechts één keer worden gebruikt. Als uw LMS daar langer over deed, of als de registratie opnieuw werd geprobeerd nadat deze al was gelukt, wordt het token geweigerd. Genereer een nieuwe URL op de FastComments LTI 1.3-configuratiepagina en begin opnieuw.

#### "Platform heeft registratie geweigerd"

Uw LMS wees de registratie-handshake af. De meest voorkomende oorzaken:

- **Tool al geregistreerd met dezelfde clientnaam.** Sommige platforms (met name D2L) weigeren een tweede registratie van "FastComments" totdat de vorige is verwijderd. Verwijder de oude tool in uw LMS en probeer het opnieuw.
- **Verkeerd veld in het LMS.** Zorg ervoor dat u de URL hebt geplakt in het **registration / tool initiation registration endpoint** veld, niet in het launch URL- of login URL-veld.
- **Het LMS ondersteunt Dynamic Registration niet daadwerkelijk.** Oudere versies van Moodle en Blackboard adverteren LTI 1.3 maar staan alleen handmatige configuratie toe. Controleer de documentatie van uw platform.

#### "Kon platformconfiguratie niet ophalen"

FastComments kon het openid-configuration-document van uw LMS niet lezen. Dit is zeldzaam en betekent meestal dat het LMS een onjuist of onbereikbaar discovery URL heeft opgegeven. Neem contact op met de ondersteuning van uw LMS.

#### "Launch toont "Configuratie niet gevonden""

Of de configuratie in FastComments is verwijderd, of de launch kwam van een `iss`/`client_id`-paar dat we niet herkennen. Als u verwijderd en opnieuw geregistreerd hebt, geef uw LMS dan de opdracht om de FastComments-tool te verwijderen en opnieuw toe te voegen zodat deze de nieuwe `client_id` krijgt.

#### "Launch toont "Deployment not registered""

U startte FastComments vanaf een Brightspace/Moodle/Blackboard-deployment die verschilde van degene waarin het oorspronkelijk werd gestart. FastComments koppelt de `deployment_id` bij de eerste start als een beveiligingscontrole. Om een nieuwe deployment toe te voegen onder dezelfde client, neem contact op met support - wij voegen de deployment ID toe aan de configuratie.

#### "Launch toont "Unsupported message_type""

Het LMS stuurde een LTI-bericht dat FastComments niet verwerkt (bijv. `LtiSubmissionReviewRequest`). FastComments ondersteunt alleen de standaard resource-link launch en deep-linking flows. Neem contact op als u een specifiek berichttype toegevoegd wilt hebben.

#### "Iframe past niet automatisch van grootte aan"

De meeste LMS'en schalen LTI-iframes automatisch. Als dat bij uw LMS niet het geval is, controleer dan of de launch-instellingen van het LMS toestaan dat de tool postMessage-events naar het bovenliggende frame stuurt. FastComments zendt zowel Canvas-stijl (`lti.frameResize`) als IMS-specificatie (`org.imsglobal.lti.frameResize`) resize-berichten uit.