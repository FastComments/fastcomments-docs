D2L Brightspace biedt Dynamic Registration via de LTI Advantage-beheerinterface. Je hebt beheerdersrechten nodig.

#### Open the Registration Screen

1. Meld je aan bij je Brightspace-instantie als beheerder.
2. Navigeer naar **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klik op **Register Tool**. (De directe URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Je ziet een registratieformulier. Het belangrijkste veld is **Tool initiation registration endpoint** (sommige Brightspace-versies noemen het "Tool Initiation Registration URL").

Plak de FastComments-registratie-URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">haal het hier op</a>) in dat veld. Laat de andere velden leeg - ze worden automatisch ingevuld door FastComments tijdens de registratie-handshake.

Klik op **Register**.

#### Keur de tool goed

Brightspace opent een pop-up die met FastComments communiceert, sleutels ruilt en een bevestigingsscherm toont. De pop-up sluit automatisch wanneer de registratie is voltooid.

De nieuwe tool verschijnt in je LTI Advantage-toollijst. Standaard markeert Brightspace nieuwe tools als **disabled** - zet de schakelaar naar **enabled** zodat je cursussen deze kunnen gebruiken.

#### Voeg een Deployment toe

In Brightspace hebben LTI-tools een **deployment** nodig voordat ze in cursussen kunnen worden gebruikt:

1. Open de zojuist geregistreerde FastComments-tool.
2. Klik op **View Deployments** > **New Deployment**.
3. Geef de deployment een naam (bijv. "FastComments - All Courses"), kies de organisatie-eenheden waarin het beschikbaar moet zijn, en sla op.

Na de eerste start via deze deployment koppelt FastComments de `deployment_id` aan zijn configuratierecord - volgende starts vanuit een andere deployment onder dezelfde client worden geweigerd, tenzij je opnieuw registreert.