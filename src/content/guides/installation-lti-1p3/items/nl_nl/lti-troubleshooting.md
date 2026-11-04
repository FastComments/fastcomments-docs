#### "Registratietoken niet gevonden, verlopen of al gebruikt"

Het token in je registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal het hier op</a>) is 30 minuten geldig en kan slechts één keer worden gebruikt. Als je LMS er langer over deed, of als de registratie opnieuw werd geprobeerd nadat deze al was geslaagd, wordt het token geweigerd. Genereer een nieuwe URL op de FastComments LTI 1.3 Configuratiepagina en begin opnieuw.

#### "Platform heeft registratie geweigerd"

Je LMS heeft de registratie-handshake geweigerd. De meest voorkomende oorzaken:

- **Tool al geregistreerd met dezelfde clientnaam.** Sommige platforms (met name D2L) weigeren een tweede registratie van "FastComments" totdat de vorige is verwijderd. Verwijder de oude tool in je LMS en probeer het opnieuw.
- **Verkeerd veld in het LMS.** Zorg ervoor dat je de URL hebt geplakt in het veld **registration / tool initiation registration endpoint**, niet in het veld voor de launch-URL of login-URL.
- **Het LMS ondersteunt eigenlijk geen Dynamic Registration.** Oudere Moodle- en Blackboard-versies adverteren met LTI 1.3 maar staan alleen handmatige configuratie toe. Raadpleeg de documentatie van je platform.

#### "Kan platformconfiguratie niet ophalen"

FastComments kon het openid-configuration-document van je LMS niet lezen. Dit is zeldzaam en betekent meestal dat het LMS een onjuist gevormde of onbereikbare discovery-URL heeft opgegeven. Neem contact op met de support van je LMS.

#### Launch toont "Configuration not found"

Ofwel is de configuratie in FastComments verwijderd, of de launch kwam van een `iss`/`client_id`-combinatie die we niet herkennen. Als je hebt verwijderd en opnieuw geregistreerd, geef je LMS dan de opdracht om de FastComments-tool te verwijderen en opnieuw toe te voegen, zodat deze de nieuwe client_id ontvangt.

#### Launch toont "Deployment not registered"

Je hebt FastComments gestart vanuit een Brightspace/Moodle/Blackboard-deployment die anders is dan degene waarin het voor het eerst werd gestart. FastComments pinnt de `deployment_id` bij de eerste launch als beveiligingscontrole. Om een nieuwe deployment onder dezelfde client toe te voegen, neem je contact op met support - wij voegen de deployment-ID toe aan de configuratie.

#### Launch toont "Unsupported message_type"

Het LMS heeft een LTI-bericht verzonden dat FastComments niet verwerkt (bijv. `LtiSubmissionReviewRequest`). FastComments ondersteunt alleen de standaard resource-link launch en deep-linking-flows. Neem contact op als je een specifiek berichttype toegevoegd nodig hebt.

#### Iframe past grootte niet aan

De meeste LMS'en passen de grootte van LTI-iframes automatisch aan. Als dat bij jou niet het geval is, controleer dan of de launch-instellingen van het LMS de tool toestaan postMessage-events naar het bovenliggende frame te sturen. FastComments stuurt zowel Canvas-stijl (`lti.frameResize`) als IMS-spec (`org.imsglobal.lti.frameResize`) resize-berichten.