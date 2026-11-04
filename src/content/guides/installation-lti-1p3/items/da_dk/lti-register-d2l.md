D2L Brightspace eksponerer Dynamic Registration via LTI Advantage-administrationsgrænsefladen. Du skal have administratoradgang.

#### Åbn registreringsskærmen

1. Log ind på din Brightspace-instans som administrator.
2. Naviger til **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Klik på **Register Tool**. (Den direkte URL er `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Indsæt URL'en

Du vil se en registreringsformular. Nøglefeltet er **Tool initiation registration endpoint** (nogle Brightspace-versioner kalder det "Tool Initiation Registration URL").

Indsæt FastComments-registrerings-URL'en (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">hent den her</a>) i det felt. Lad de andre felter stå tomme - de udfyldes automatisk af FastComments under registreringssammenkoblingen.

Klik på **Register**.

#### Godkend værktøjet

Brightspace åbner en popup, som kommunikerer med FastComments, udveksler nøgler og viser en bekræftelsesskærm. Popuppen lukker sig selv, når registreringen er fuldført.

Det nye værktøj vises i din LTI Advantage-værktøjsliste. Som standard markerer Brightspace nye værktøjer som **disabled** - slå kontakten til **enabled**, så dine kurser kan bruge det.

#### Tilføj en deployment

I Brightspace kræver LTI-værktøjer en **deployment**, før de kan bruges i kurser:

1. Åbn det nyregistrerede FastComments-værktøj.
2. Klik på **View Deployments** > **New Deployment**.
3. Giv deploymenten et navn (f.eks. "FastComments - All Courses"), vælg de org-enheder, den skal være tilgængelig i, og gem.

Efter den første opstart gennem denne deployment fastlåser FastComments `deployment_id` i sin konfigurationspost - efterfølgende opstart fra en anden deployment under samme klient vil blive afvist, medmindre du registrerer igen.