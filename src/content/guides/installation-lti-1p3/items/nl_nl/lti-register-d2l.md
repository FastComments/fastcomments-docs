D2L Brightspace biedt Dynamic Registration via de LTI Advantage-beheerinterface. U hebt beheerdersrechten nodig.

#### Open the Registration Screen

1. Meld u aan bij uw Brightspace-instantie als beheerder.
2. Navigeer naar **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klik op **Register Tool**. (De directe URL is `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

U ziet een registratieformulier. Het belangrijkste veld is **Tool initiation registration endpoint** (sommige Brightspace-versies noemen het "Tool Initiation Registration URL").

Plak de FastComments-registratie-URL in dat veld. Laat de andere velden leeg - deze worden tijdens de registratiehandshake door FastComments automatisch ingevuld.

Klik op **Register**.

#### Approve the Tool

Brightspace opent een popup die met FastComments communiceert, sleutels uitwisselt en een bevestigingsscherm toont. De popup sluit zichzelf wanneer de registratie voltooid is.

De nieuwe tool verschijnt in uw LTI Advantage-toollijst. Standaard markeert Brightspace nieuwe tools als **disabled** - zet de schakelaar op **enabled** zodat uw cursussen deze kunnen gebruiken.

#### Add a Deployment

In Brightspace hebben LTI-tools een **deployment** nodig voordat ze in cursussen gebruikt kunnen worden:

1. Open de recent geregistreerde FastComments-tool.
2. Klik op **View Deployments** > **New Deployment**.
3. Geef de deployment een naam (bijv. "FastComments - All Courses"), selecteer de organisatie-eenheden waarin deze beschikbaar moet zijn, en sla op.

Na de eerste start via deze deployment koppelt FastComments de `deployment_id` aan zijn configuratierecord - volgende starts vanaf een andere deployment onder dezelfde client worden geweigerd tenzij u opnieuw registreert.