D2L Brightspace espone la registrazione dinamica tramite l'interfaccia amministrativa LTI Advantage. Avrai bisogno dell'accesso amministratore.

#### Apri la schermata di registrazione

1. Accedi alla tua istanza Brightspace come amministratore.
2. Vai a **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Clicca **Register Tool**. (L'URL diretto è `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Incolla l'URL

Vedrai un modulo di registrazione. Il campo chiave è **Tool initiation registration endpoint** (in alcune versioni di Brightspace è etichettato come "Tool Initiation Registration URL").

Incolla l'URL di registrazione di FastComments in quel campo. Lascia gli altri campi vuoti - vengono compilati automaticamente da FastComments durante la stretta di mano di registrazione.

Clicca **Register**.

#### Approva lo strumento

Brightspace apre una finestra popup che comunica con FastComments, scambia chiavi e mostra una schermata di conferma. La popup si chiude automaticamente quando la registrazione è completata.

Il nuovo strumento appare nell'elenco strumenti LTI Advantage. Per impostazione predefinita Brightspace contrassegna i nuovi strumenti come **disabled** - sposta l'interruttore su **enabled** in modo che i tuoi corsi possano usarlo.

#### Aggiungi una distribuzione

In Brightspace, gli strumenti LTI richiedono una **deployment** prima di poter essere usati nei corsi:

1. Apri il nuovo strumento FastComments appena registrato.
2. Clicca **View Deployments** > **New Deployment**.
3. Assegna alla distribution un nome (es. "FastComments - All Courses"), seleziona le unità organizzative in cui dovrebbe essere disponibile e salva.

Dopo il primo avvio tramite questa distribuzione, FastComments fissa il `deployment_id` nel suo record di configurazione - avvii successivi da una distribuzione diversa sotto lo stesso client verranno rifiutati a meno che tu non ti registri di nuovo.