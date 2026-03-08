1. Accedi a FastComments e vai a <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">My Account > Canvas LTI Config</a>.
2. Inserisci un **Configuration Name** e il tuo **Platform URL** (e.g. `https://yourschool.instructure.com`), quindi clicca **Create Configuration**. La procedura guidata avanza allo Step 2 e mostra la tua **Configuration URL**.
3. In Canvas, vai su **Admin > Developer Keys > + Developer Key > LTI Key**. Imposta **Method** su "Enter URL" e incolla la Configuration URL. Salva la chiave e imposta il suo **State** su **ON**.
4. Copia il numero **Client ID** dalla Developer Keys table in Canvas. Torna in FastComments, incollalo nel campo **Client ID** e clicca **Save & Continue**.
5. Rivedi il riepilogo della configurazione e clicca **Enable Integration** per andare in produzione.
6. Nel tuo corso Canvas, vai su **Settings > Navigation**, trova **FastComments** e attivalo. I commenti appariranno come voce di navigazione del corso.