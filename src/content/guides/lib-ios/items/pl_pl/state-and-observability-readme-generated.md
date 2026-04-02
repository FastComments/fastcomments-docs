Obie `FastCommentsSDK` i `FastCommentsFeedSDK` są klasami `ObservableObject` z właściwościami `@Published`. Możesz obserwować je w swoich widokach SwiftUI, aby uzyskać reaktywne aktualizacje interfejsu użytkownika.

### Właściwości publikowane FastCommentsSDK

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Łączna liczba komentarzy na serwerze |
| `newRootCommentCount` | `Int` | Buforowane nowe komentarze (gdy `showLiveRightAway` jest false) |
| `currentUser` | `UserSessionInfo?` | Bieżący uwierzytelniony użytkownik |
| `isSiteAdmin` | `Bool` | Czy bieżący użytkownik jest administratorem witryny |
| `isClosed` | `Bool` | Czy wątek komentarzy jest zamknięty |
| `hasBillingIssue` | `Bool` | Czy istnieje problem z rozliczeniami |
| `isLoading` | `Bool` | Czy trwa żądanie sieciowe |
| `hasMore` | `Bool` | Czy istnieją kolejne strony komentarzy |
| `blockingErrorMessage` | `String?` | Błąd uniemożliwiający działanie interfejsu |
| `warningMessage` | `String?` | Nieblokujący komunikat ostrzegawczy |
| `isDemo` | `Bool` | Czy działa w trybie demo |
| `commentsVisible` | `Bool` | Przełącznik widoczności komentarzy |
| `toolbarEnabled` | `Bool` | Czy pasek narzędzi formatowania jest widoczny |

### Właściwości publikowane FastCommentsFeedSDK

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Aktualnie załadowane posty kanału |
| `hasMore` | `Bool` | Czy istnieją kolejne strony |
| `currentUser` | `UserSessionInfo?` | Bieżący uwierzytelniony użytkownik |
| `blockingErrorMessage` | `String?` | Blokujący komunikat o błędzie |
| `isLoading` | `Bool` | Czy trwa żądanie sieciowe |
| `newPostsCount` | `Int` | Liczba nowych postów od ostatniego załadowania |

### Drzewo komentarzy

Drzewo komentarzy jest dostępne przez `sdk.commentsTree`:

```swift
// Płaska lista widocznych węzłów do renderowania
sdk.commentsTree.visibleNodes

// Wyszukaj komentarz po ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---