### Veličina stranice

```swift
// Komentari: podrazumevano 30
sdk.pageSize = 50

// Feed: podrazumevano 10
feedSDK.pageSize = 20
```

### Učitavanje više komentara

UI automatski prikazuje kontrole za paginaciju. Takođe možete pokrenuti paginaciju programatski:

```swift
// Učitaj sledeću stranicu
try await sdk.loadMore()

// Učitaj sve preostalo (onemogućeno ako ima >2000 komentara zbog performansi)
try await sdk.loadAll()

// Proveri stanje
sdk.hasMore            // Da li postoje još stranica
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginacija podkomentara

Ugnježdeni odgovori se učitavaju po potrebi. Kada korisnik proširi nit, prvih 5 odgovora se učitava. Kontrola "učitaj više odgovora" pojavljuje se ako ima još. Ovo se automatski rešava u korisničkom interfejsu.

---
---