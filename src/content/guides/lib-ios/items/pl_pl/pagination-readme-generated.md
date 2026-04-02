### Rozmiar strony

```swift
// Komentarze: domyślnie 30
sdk.pageSize = 50

// Kanał: domyślnie 10
feedSDK.pageSize = 20
```

### Ładowanie kolejnych komentarzy

Interfejs użytkownika automatycznie wyświetla kontrolki paginacji. Możesz też wywołać paginację programowo:

```swift
// Załaduj następną stronę
try await sdk.loadMore()

// Załaduj wszystkie pozostałe (wyłączone jeśli >2000 komentarzy ze względów wydajnościowych)
try await sdk.loadAll()

// Sprawdź stan
sdk.hasMore            // Czy istnieją kolejne strony
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Paginacja komentarzy podrzędnych

Zagnieżdżone odpowiedzi ładują się leniwie. Gdy użytkownik rozwija wątek, ładowane są pierwsze 5 odpowiedzi podrzędnych. Pojawia się kontrolka "załaduj więcej odpowiedzi", jeśli istnieją kolejne. Jest to obsługiwane automatycznie przez interfejs użytkownika.

---
---