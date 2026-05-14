D2L Brightspace espone la Registrazione Dinamica tramite l'interfaccia di amministrazione LTI Advantage. Avrai bisogno di accesso amministrativo.

#### Open the Registration Screen

1. Accedi alla tua istanza Brightspace come amministratore.
2. Vai a **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Fai clic su **Register Tool**. (L'URL diretto è `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Vedrai un modulo di registrazione. Il campo chiave è **Tool initiation registration endpoint** (alcune versioni di Brightspace lo etichettano come "Tool Initiation Registration URL").

Incolla l'URL di registrazione di FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">ottienilo qui</a>) in quel campo. Lascia gli altri campi vuoti - vengono compilati automaticamente da FastComments durante lo scambio di registrazione.

Fai clic su **Register**.

#### Approve the Tool

Brightspace apre una finestra pop-up che comunica con FastComments, scambia le chiavi e mostra una schermata di conferma. La finestra pop-up si chiude automaticamente quando la registrazione è completata.

Il nuovo strumento appare nell'elenco degli strumenti LTI Advantage. Per impostazione predefinita Brightspace contrassegna i nuovi strumenti come **disabled** - attiva l'interruttore su **enabled** affinché i tuoi corsi possano utilizzarlo.

#### Add a Deployment

In Brightspace, gli strumenti LTI necessitano di un **deployment** prima di poter essere utilizzati nei corsi:

1. Apri lo strumento FastComments appena registrato.
2. Fai clic su **View Deployments** > **New Deployment**.
3. Dai al deployment un nome (es. "FastComments - All Courses"), seleziona le unità organizzative in cui deve essere disponibile e salva.

Dopo il primo avvio tramite questo deployment, FastComments associa il `deployment_id` al suo record di configurazione - i successivi avvii da un deployment diverso sotto lo stesso client saranno rifiutati a meno che non ti registri nuovamente.