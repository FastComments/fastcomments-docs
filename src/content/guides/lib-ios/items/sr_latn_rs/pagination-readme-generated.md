### Page Size

```swift
// Komentari: podrazumevano 30
sdk.pageSize = 50

// Feed: podrazumevano 10
feedSDK.pageSize = 20
```

### Loading More Comments

UI automatski prikazuje kontrole paginacije. Takođe možete programatski pokrenuti paginaciju:

```swift
// Učitaj sledeću stranicu
try await sdk.loadMore()

// Učitaj sve preostale (onemogućeno ako ima >2000 komentara zbog performansi)
try await sdk.loadAll()

// Proveri stanje
sdk.hasMore            // Da li postoje još stranica
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Child Comment Pagination

Ugnježdeni odgovori se učitavaju na zahtev. Kada korisnik proširi nit, učitava se prvih 5 podkomentara. Pojavi se kontrola "učitaj još odgovora" ako ima više. Ovo se automatski obavlja u UI-ju.

---
---