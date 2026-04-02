### Paginagrootte

```swift
// Opmerkingen: standaard 30
sdk.pageSize = 50

// Feed: standaard 10
feedSDK.pageSize = 20
```

### Meer opmerkingen laden

De UI toont pagineringsbedieningselementen automatisch. U kunt paginering ook programmatisch activeren:

```swift
// Volgende pagina laden
try await sdk.loadMore()

// Alle resterende laden (uitgeschakeld als >2000 opmerkingen omwille van prestaties)
try await sdk.loadAll()

// Controleer status
sdk.hasMore            // Of er meer pagina's bestaan
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginering van kindreacties

Geneste reacties worden lui geladen. Wanneer een gebruiker een thread uitklapt, worden de eerste 5 onderliggende reacties geladen. Er verschijnt een bediening "laad meer reacties" als er meer zijn. Dit wordt automatisch door de UI afgehandeld.

---
---