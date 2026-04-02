### Velikost strani

```swift
// Komentarji: privzeto 30
sdk.pageSize = 50

// Feed: privzeto 10
feedSDK.pageSize = 20
```

### Nalaganje več komentarjev

UI samodejno prikaže krmilnike za paginacijo. Paginacijo lahko sprožite tudi programatično:

```swift
// Naloži naslednjo stran
try await sdk.loadMore()

// Naloži vse preostale (onemogočeno, če >2000 komentarjev zaradi zmogljivosti)
try await sdk.loadAll()

// Preveri stanje
sdk.hasMore            // Ali obstajajo še strani
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginacija podrejenih komentarjev

Gnezdeni odgovori se nalagajo po potrebi. Ko uporabnik razširi nit, se naloži prvih 5 podrejenih odgovorov. Če obstajajo dodatni odgovori, se prikaže kontrola "load more replies". To upravlja uporabniški vmesnik samodejno.

---
---