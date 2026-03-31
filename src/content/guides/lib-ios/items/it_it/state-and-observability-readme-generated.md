---
Sia `FastCommentsSDK` che `FastCommentsFeedSDK` sono classi `ObservableObject` con proprietà `@Published`. Puoi osservare queste proprietà nelle tue view SwiftUI per aggiornamenti reattivi dell'interfaccia utente.

### Proprietà @Published di FastCommentsSDK

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Conteggio totale dei commenti sul server |
| `newRootCommentCount` | `Int` | Commenti nuovi in buffer (quando `showLiveRightAway` è false) |
| `currentUser` | `UserSessionInfo?` | Utente autenticato corrente |
| `isSiteAdmin` | `Bool` | Se l'utente corrente è amministratore del sito |
| `isClosed` | `Bool` | Se il thread dei commenti è chiuso |
| `hasBillingIssue` | `Bool` | Se c'è un problema di fatturazione |
| `isLoading` | `Bool` | Se è in corso una richiesta di rete |
| `hasMore` | `Bool` | Se esistono altre pagine di commenti |
| `blockingErrorMessage` | `String?` | Errore che impedisce il funzionamento dell'interfaccia |
| `warningMessage` | `String?` | Messaggio di avviso non bloccante |
| `isDemo` | `Bool` | Se in esecuzione in modalità demo |
| `commentsVisible` | `Bool` | Interruttore per la visibilità dei commenti |
| `toolbarEnabled` | `Bool` | Se la barra degli strumenti di formattazione è mostrata |

### Proprietà @Published di FastCommentsFeedSDK

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Post del feed attualmente caricati |
| `hasMore` | `Bool` | Se esistono altre pagine |
| `currentUser` | `UserSessionInfo?` | Utente autenticato corrente |
| `blockingErrorMessage` | `String?` | Messaggio di errore bloccante |
| `isLoading` | `Bool` | Se è in corso una richiesta di rete |
| `newPostsCount` | `Int` | Numero di nuovi post dall'ultimo caricamento |

### Albero dei commenti

L'albero dei commenti è accessibile tramite `sdk.commentsTree`:

```swift
// Elenco piatto di nodi visibili per il rendering
sdk.commentsTree.visibleNodes

// Ricerca di un commento per ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---