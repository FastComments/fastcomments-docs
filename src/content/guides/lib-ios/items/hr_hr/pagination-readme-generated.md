### Veličina stranice

```swift
// Komentari: zadano 30
sdk.pageSize = 50

// Feed: zadano 10
feedSDK.pageSize = 20
```

### Učitavanje više komentara

UI automatski prikazuje kontrole za paginaciju. Također možete programatski pokrenuti paginaciju:

```swift
// Učitaj sljedeću stranicu
try await sdk.loadMore()

// Učitaj sve preostale (onemogućeno ako >2000 komentara zbog performansi)
try await sdk.loadAll()

// Provjeri stanje
sdk.hasMore            // Postoje li još stranica
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginacija podkomentara

Ugniježđeni odgovori se učitavaju lenjo. Kad korisnik proširi nit, prvih 5 podkomentara se učita. Pojavljuje se kontrola "učitaj još odgovora" ako ih ima više. Time se rukuje automatski u UI-ju.

---
---