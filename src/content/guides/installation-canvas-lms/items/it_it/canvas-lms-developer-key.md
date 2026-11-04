#### Apri le Developer Keys in Canvas

Accedi a Canvas come amministratore. Vai su **Admin** (nella barra laterale sinistra) > seleziona il tuo account > **Developer Keys**.

#### Crea una LTI Developer Key

Fai clic su **+ Developer Key** e seleziona **LTI Key**.

Nel modulo di configurazione:

1. Nel campo **Redirect URIs** (lato sinistro), incolla il **Launch URL** dalla pagina di configurazione di FastComments.
2. A destra, imposta **Method** su **Enter URL**.
3. Incolla il **Configuration URL** copiato da FastComments nel campo **JSON URL**.
4. Canvas caricherà automaticamente la configurazione LTI.
5. Assegna un nome alla chiave (ad es. "FastComments").
6. Fai clic su **Save**.

#### Abilita la Developer Key

Dopo il salvataggio, la nuova chiave apparirà nella tabella Developer Keys con **State** impostato su **OFF**. Fai clic sull'interruttore per impostarlo su **ON**. Canvas potrebbe chiederti di confermare — fai clic su **Allow** per abilitare la chiave.

#### Copia il Client ID

La tabella Developer Keys mostra un **Client ID** numerico nella colonna Details (ad es. `17000000000042`). Copia questo numero - ti servirà nel passaggio successivo.