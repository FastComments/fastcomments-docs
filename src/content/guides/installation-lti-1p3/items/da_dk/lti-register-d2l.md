---
D2L Brightspace gør Dynamic Registration tilgængelig via LTI Advantage-administrationsgrænsefladen. Du skal have administratoradgang.

#### Åbn registreringsskærmen

1. Log ind på din Brightspace-instans som administrator.
2. Gå til **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klik på **Register Tool**. (Den direkte URL er `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Indsæt URL'en

Du vil se en registreringsformular. Nøglefeltet er **Tool initiation registration endpoint** (nogle Brightspace-versioner mærker det som "Tool Initiation Registration URL").

Indsæt FastComments-registrerings-URL'en i det felt. Lad de andre felter være tomme - de udfyldes automatisk af FastComments under registreringshåndtrykket.

Klik på **Register**.

#### Godkend værktøjet

Brightspace åbner en popup, som kommunikerer med FastComments, udveksler nøgler og viser en bekræftelsesskærm. Popup'en lukker automatisk, når registreringen er fuldført.

Det nye værktøj vises i din LTI Advantage-værktøjsliste. Som standard markerer Brightspace nye værktøjer som **disabled** - slå toggleknappen til **enabled**, så dine kurser kan bruge det.

#### Tilføj en deployment

I Brightspace kræver LTI-værktøjer en **deployment**, før de kan bruges i kurser:

1. Åbn det nyregistrerede FastComments-værktøj.
2. Klik på **View Deployments** > **New Deployment**.
3. Giv deployment'en et navn (f.eks. "FastComments - All Courses"), vælg de organisatoriske enheder, den skal være tilgængelig i, og gem.

Efter det første igangsæt gennem denne deployment binder FastComments `deployment_id` til sin konfigurationspost - efterfølgende igangsæt fra en anden deployment under samme client vil blive afvist, medmindre du genregistrerer.

---