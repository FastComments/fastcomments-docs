I badge utente di FastComments vengono configurati dagli amministratori che hanno il permesso `Customize Data`.

Questo si fa tramite [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) nel pannello di amministrazione.

Quando a un utente viene assegnato un badge, questo viene visualizzato sul suo profilo e nei suoi commenti.

Quando si aggiunge un badge possiamo impostare una `Display Label`, che è il nome che l'utente vede associato al badge. Ad esempio, se aggiungiamo un badge `Comment Count`
probabilmente non vorremo mostrare quel nome tecnico perché è piuttosto scialbo. Potremmo chiamarlo `Super Member` o qualcosa di simile. I badge possono anche accumularsi e sostituirsi a vicenda, come spiegheremo
più avanti in questo documento.

I badge hanno anche soglie configurabili.

I badge possono essere creati e poi disabilitati deselezionando `Enabled`. Disabilitare un badge significa che non verrà più assegnato automaticamente e non verrà mostrato nel menu Award Manual Badge, ma
gli utenti manterranno il badge.

### Tipi di visualizzazione dei badge

I badge possono essere immagini o badge testuali, che supportano alcune opzioni di base di stile (colore del testo, colore di sfondo e colore del bordo). Puoi anche stilizzare i badge tramite CSS.

I badge immagine possono essere file GIF per mostrare badge animati.

### Suggerimento - Non rimuovere i badge!

Agli utenti piacciono molto i badge. Ci tengono spesso davvero, anche se si tratta di un bug aggiunto per errore e vuoi cambiare l'icona del badge.

Se abbiamo imparato qualcosa, è estremamente difficile togliere qualcosa agli utenti. Rimuovere un badge perché, come proprietario del sito, non ti piace più o vuoi apportare modifiche, può causare una reazione di utenti molto arrabbiati che potrebbero abbandonare improvvisamente il tuo sito per frustrazione. Per questa ragione
`Delete` non era neppure un'opzione nei primi mesi dopo il rilascio di questa funzionalità - tuttavia alla fine abbiamo dovuto aggiungerla. Ma per favore, usa l'eliminazione con cautela. Abbiamo
visto molti utenti di lunga data, attivi da anni, diventare molto frustrati e lasciare le loro community perché gli amministratori hanno deciso di eliminare un badge.

Se devi smettere di usare un badge, ti suggeriamo di semplicemente disabilitarlo in modo che gli utenti mantengano il badge.

### Rielaborazione dei badge

Quando un badge viene aggiunto o modificato, il sistema controllerà retroattivamente chiunque abbia interagito con il tuo sito per verificare se debbano ottenere il badge. Questo sarà
visibile nella pagina Badges nel pannello di amministrazione, poiché verrà mostrato un indicatore di caricamento al posto del numero di utenti che possiedono il badge. Questo perché il numero di utenti
è in fase di determinazione.

### Vedere chi ha un badge

Nella lista Badges ogni link ha un'opzione `View Users` per mostrare l'elenco degli utenti che hanno guadagnato o a cui è stato assegnato manualmente un badge.