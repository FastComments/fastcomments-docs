### Dimensione della pagina

```swift
// Commenti: predefinito 30
sdk.pageSize = 50

// Feed: predefinito 10
feedSDK.pageSize = 20
```

### Caricamento di altri commenti

L'interfaccia utente mostra automaticamente i controlli di impaginazione. Puoi anche avviare l'impaginazione programmaticamente:

```swift
// Carica la pagina successiva
try await sdk.loadMore()

// Carica tutti i rimanenti (disabilitato se >2000 commenti per motivi di prestazioni)
try await sdk.loadAll()

// Controlla lo stato
sdk.hasMore            // Se esistono altre pagine
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Impaginazione dei commenti figli

Le risposte annidate vengono caricate su richiesta (lazy). Quando un utente espande un thread, vengono caricate le prime 5 risposte figlie. Compare un controllo "carica altre risposte" se ne esistono altre. Questo viene gestito automaticamente dall'interfaccia utente.