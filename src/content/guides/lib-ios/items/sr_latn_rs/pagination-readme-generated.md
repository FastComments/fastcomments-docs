### Veličina stranice

```swift
// Komentari: podrazumevano 30
sdk.pageSize = 50

// Feed: podrazumevano 10
feedSDK.pageSize = 20
```

### Učitavanje više komentara

UI automatski prikazuje kontrole za paginaciju. Takođe možete pokrenuti paginaciju programski:

```swift
// Učitaj sledeću stranicu
try await sdk.loadMore()

// Učitaj sve preostalo (onemogućeno ako ima >2000 komentara, radi performansi)
try await sdk.loadAll()

// Proveri stanje
sdk.hasMore            // Da li postoje dodatne stranice
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginacija podređenih komentara

Ugnježdeni odgovori se učitavaju lenjo. Kada korisnik proširi nit, učitava se prvih 5 podređenih komentara. Ako ih ima više, pojavljuje se kontrola "učitaj još odgovora". Ovo UI automatski rešava.

---