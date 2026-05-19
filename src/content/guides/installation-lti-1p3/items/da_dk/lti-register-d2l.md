D2L Brightspace udstiller Dynamisk Registrering gennem LTI Advantage administrationsgrænsefladen. Du skal have administratoradgang.

#### Open the Registration Screen

1. Log ind på din Brightspace-instans som administrator.
2. Gå til **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klik **Register Tool**. (Den direkte URL er `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Du vil se en registreringsformular. Nøglen er feltet **Tool initiation registration endpoint** (some Brightspace versions label it "Tool Initiation Registration URL").

Indsæt FastComments-registrerings-URL'en (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>) i det felt. Lad de andre felter stå tomme - de udfyldes automatisk af FastComments under registreringshåndtrykket.

Klik **Register**.

#### Approve the Tool

Brightspace åbner en popup, som kommunikerer med FastComments, udveksler nøgler og viser en bekræftelsesskærm. Popuppen lukker sig selv, når registreringen er fuldført.

Det nye værktøj vises i din LTI Advantage værktøjsliste. Som standard markerer Brightspace nye værktøjer som **disabled** - slå toggle-knappen om til **enabled**, så dine kurser kan bruge det.

#### Add a Deployment

I Brightspace skal LTI-værktøjer have en **deployment**, før de kan bruges i kurser:

1. Åbn det nyregistrerede FastComments-værktøj.
2. Klik **View Deployments** > **New Deployment**.
3. Giv deploymenten et navn (f.eks. "FastComments - All Courses"), vælg de organisationsenheder, den skal være tilgængelig i, og gem.

Efter den første start via denne deployment, binder FastComments `deployment_id` til sin konfigurationspost - efterfølgende starter fra en anden deployment under samme klient vil blive afvist, medmindre du registrerer igen.