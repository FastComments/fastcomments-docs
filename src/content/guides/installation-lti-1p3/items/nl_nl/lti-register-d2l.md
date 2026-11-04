D2L Brightspace stelt Dynamic Registration beschikbaar via de LTI Advantage admin-interface. Je hebt administrator-toegang nodig.

#### Open het Registratiescherm

1. Log in op je Brightspace-instantie als admin.
2. Navigeer naar **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klik op **Register Tool**. (De directe URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Plak de URL

Je ziet een registratieformulier. Het belangrijkste veld is **Tool initiation registration endpoint** (sommige Brightspace-versies noemen het "Tool Initiation Registration URL").

Plak de FastComments-registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal hem hier op</a>) in dat veld. Laat de andere velden leeg - deze worden automatisch gevuld door FastComments tijdens de registratie-handshake.

Klik op **Register**.

#### Keur de Tool Goed

Brightspace opent een popup die met FastComments communiceert, sleutels uitwisselt en een bevestigingsscherm toont. De popup sluit zichzelf wanneer de registratie is voltooid.

De nieuwe tool verschijnt in je LTI Advantage-toollijst. Standaard markeert Brightspace nieuwe tools als **disabled** - zet de schakelaar op **enabled** zodat je cursussen deze kunnen gebruiken.

#### Voeg een Deployment Toe

In Brightspace hebben LTI-tools een **deployment** nodig voordat ze in cursussen kunnen worden gebruikt:

1. Open de zojuist geregistreerde FastComments-tool.
2. Klik op **View Deployments** > **New Deployment**.
3. Geef de deployment een naam (bijv. "FastComments - All Courses"), kies de org-units waarin deze beschikbaar moet zijn en sla op.

Na de eerste lancering via deze deployment koppelt FastComments de `deployment_id` aan zijn configuratierecord - daaropvolgende lanceringen vanaf een andere deployment onder dezelfde client worden afgewezen, tenzij je opnieuw registreert.