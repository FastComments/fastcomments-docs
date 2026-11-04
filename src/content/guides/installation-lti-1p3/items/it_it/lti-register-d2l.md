D2L Brightspace espone la Dynamic Registration tramite l'interfaccia di amministrazione LTI Advantage. Sarà necessario l'accesso come amministratore.

#### Aprire la Schermata di Registrazione

1. Accedi alla tua istanza Brightspace come amministratore.
2. Vai su **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Clicca su **Register Tool**. (L'URL diretto è `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Incollare l'URL

Vedrai un modulo di registrazione. Il campo chiave è **Tool initiation registration endpoint** (alcune versioni di Brightspace lo etichettano come "Tool Initiation Registration URL").

Incolla l'URL di registrazione di FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">lo trovi qui</a>) in quel campo. Lascia vuoti gli altri campi - vengono compilati automaticamente da FastComments durante l'handshake di registrazione.

Clicca su **Register**.

#### Approvare lo Strumento

Brightspace apre un popup che comunica con FastComments, scambia le chiavi e mostra una schermata di conferma. Il popup si chiude da solo al termine della registrazione.

Il nuovo strumento appare nell'elenco degli strumenti LTI Advantage. Per impostazione predefinita, Brightspace contrassegna i nuovi strumenti come **disabled** - attiva l'interruttore su **enabled** affinché i tuoi corsi possano utilizzarlo.

#### Aggiungere un Deployment

In Brightspace, gli strumenti LTI necessitano di un **deployment** prima di poter essere utilizzati nei corsi:

1. Apri lo strumento FastComments appena registrato.
2. Clicca su **View Deployments** > **New Deployment**.
3. Assegna un nome al deployment (es. "FastComments - All Courses"), seleziona le unità organizzative in cui deve essere disponibile e salva.

Dopo il primo avvio tramite questo deployment, FastComments fissa il `deployment_id` nel suo record di configurazione - gli avvii successivi da un deployment diverso sotto lo stesso client verranno rifiutati a meno che tu non effettui una nuova registrazione.