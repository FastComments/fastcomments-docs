#### Vai a Canvas LTI Config

Accedi al tuo account FastComments e vai a <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.

#### Crea una Nuova LTI Configuration

Seleziona la casella di controllo **Enable LTI**. Appariranno i campi di configurazione:

- **Configuration Name** - un'etichetta opzionale per identificare questa configurazione (utile se colleghi più istanze di Canvas).
- **Platform URL** - l'URL della tua istanza Canvas (e.g. `https://yourschool.instructure.com`). Puoi lasciarlo vuoto per ora e compilarlo dopo aver creato la Developer Key.
- **Client ID** - lascialo vuoto per ora. Lo compilerai dopo aver creato la Developer Key in Canvas.
- **Deployment ID** - lascialo vuoto per ora.
- **Comment Style** - scegli tra Comments, Collab Chat, o Both. Vedi [Commenting Styles](#canvas-lms-commenting-styles) per i dettagli.

Fai clic su **Add** per creare la configurazione.

#### Copia gli URL della Configurazione

Dopo il salvataggio, appariranno tre URL:

- **Configuration URL** - lo incollerai in Canvas quando creerai la Developer Key.
- **OIDC Login URL** - usato da Canvas per il flusso di login LTI (configurato automaticamente tramite la Configuration URL).
- **Launch URL** - l'endpoint che Canvas chiama quando uno studente apre FastComments (configurato automaticamente tramite la Configuration URL).

Copia la **Configuration URL**. Ti servirà nel passaggio successivo.