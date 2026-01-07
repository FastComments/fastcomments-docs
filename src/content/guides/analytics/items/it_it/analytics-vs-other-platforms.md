Potresti trovare che le nostre metriche Analytics mostrano numeri leggermente diversi rispetto a, diciamo, Google Ads © o prodotti simili.

Per i siti con un widget di commenti per pagina, i numeri forniti da FastComments Analytics sono molto accurati, e se incorretti saranno **inferiori** al valore effettivo, ma non superiori.

Se hai una SPA potresti trovare che i numeri di FastComments Analytics sono più alti di quelli riportati dai tuoi prodotti di marketing. Questo perché il prodotto di marketing potrebbe tracciare solo quando la pagina non è caricata, ma non ogni volta che un utente fa qualcosa nella pagina che potrebbe attivare la visualizzazione di un nuovo thread di commenti, che conterebbe come caricamento pagina per FastComments Analytics.

#### Informazioni tecniche

FastComments Analytics traccia ogni caricamento di pagina e non si basa sulla casualità come ottimizzazione. Ogni caricamento di pagina risulta in un aggiornamento del conteggio in memoria in ogni thread su ogni server, che viene poi periodicamente persistito nel database tramite un'operazione atomica.
