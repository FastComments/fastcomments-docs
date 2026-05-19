#### "Registratietoken niet gevonden, verlopen of al gebruikt"

Het token in uw registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal het hier op</a>) is 30 minuten geldig en kan slechts één keer worden gebruikt. Als uw LMS daar langer over deed, of als de registratie opnieuw werd geprobeerd nadat deze al geslaagd was, wordt het token geweigerd. Genereer een nieuwe URL op de FastComments LTI 1.3 Configuration-pagina en begin opnieuw.

#### "Platform rejected registration"

Uw LMS weigerde de registratie-handshake. De meest voorkomende oorzaken:

- **Tool already registered with the same client name.** Sommige platforms (vooral D2L) weigeren een tweede registratie van "FastComments" totdat de vorige is verwijderd. Verwijder het oude tool in uw LMS en probeer het opnieuw.
- **Wrong field in the LMS.** Zorg ervoor dat u de URL hebt geplakt in het veld **registration / tool initiation registration endpoint**, niet in het launch URL- of login URL-veld.
- **The LMS doesn't actually support Dynamic Registration.** Oudere versies van Moodle en Blackboard adverteren LTI 1.3 maar staan alleen handmatige configuratie toe. Controleer de documentatie van uw platform.

#### "Failed to fetch platform configuration"

FastComments kon het openid-configuration-document van uw LMS niet lezen. Dit is zeldzaam en betekent meestal dat het LMS een foutieve of onbereikbare discovery URL heeft opgegeven. Neem contact op met de ondersteuning van uw LMS.

#### Launch shows "Configuration not found"

Ofwel is de configuratie in FastComments verwijderd, of de launch kwam van een `iss`/`client_id`-paar dat we niet herkennen. Als u de configuratie hebt verwijderd en opnieuw hebt geregistreerd, geef uw LMS de opdracht om de FastComments-tool te verwijderen en opnieuw toe te voegen zodat deze de nieuwe client_id krijgt.

#### Launch shows "Deployment not registered"

U hebt FastComments gestart vanuit een Brightspace/Moodle/Blackboard-deployment die verschilt van degene waarin het voor het eerst werd gestart. FastComments legt de `deployment_id` vast bij de eerste start als beveiligingscontrole. Om een nieuwe deployment onder dezelfde client toe te voegen, neem contact op met support - wij voegen de deployment ID toe aan de configuratie.

#### Launch shows "Unsupported message_type"

Het LMS stuurde een LTI-bericht dat FastComments niet afhandelt (bijv. `LtiSubmissionReviewRequest`). FastComments ondersteunt alleen de standaard resource-link launch- en deep-linking-flows. Neem contact op als u een specifiek berichttype toegevoegd wilt hebben.

#### Iframe doesn't resize

De meeste LMS'en schalen LTI-iframes automatisch. Als dat bij uw LMS niet het geval is, controleer dan of de launch-instellingen van het LMS toestaan dat de tool postMessage-events naar het ouderframe stuurt. FastComments stuurt zowel Canvas-stijl (`lti.frameResize`) als IMS-specificatie (`org.imsglobal.lti.frameResize`) resize-berichten.