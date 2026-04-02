### Sidestørrelse

```swift
// Kommentarer: standard 30
sdk.pageSize = 50

// Feed: standard 10
feedSDK.pageSize = 20
```

### Indlæsning af flere kommentarer

Brugergrænsefladen viser pagineringskontroller automatisk. Du kan også udløse paginering programmatisk:

```swift
// Indlæs næste side
try await sdk.loadMore()

// Indlæs alle resterende (deaktiveret hvis >2000 kommentarer af hensyn til ydeevne)
try await sdk.loadAll()

// Kontroller status
sdk.hasMore            // Whether more pages exist
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginering af underkommentarer

Indlejrede svar indlæses efter behov. Når en bruger udvider en tråd, indlæses de første 5 underkommentarer. En "indlæs flere svar"-kontrol vises, hvis der er flere. Dette håndteres automatisk af brugergrænsefladen.

---
---