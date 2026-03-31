Beide `FastCommentsSDK` en `FastCommentsFeedSDK` zijn `ObservableObject`-klassen met `@Published`-eigenschappen. Je kunt deze observeren in je SwiftUI-weergaven voor reactieve UI-updates.

### Gepubliceerde eigenschappen van FastCommentsSDK

| Eigenschap | Type | Beschrijving |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Totaal aantal reacties op de server |
| `newRootCommentCount` | `Int` | Gebufferde nieuwe reacties (wanneer `showLiveRightAway` false is) |
| `currentUser` | `UserSessionInfo?` | Huidige geauthenticeerde gebruiker |
| `isSiteAdmin` | `Bool` | Of de huidige gebruiker sitebeheerder is |
| `isClosed` | `Bool` | Of de discussiedraad gesloten is |
| `hasBillingIssue` | `Bool` | Of er een betalingsprobleem is |
| `isLoading` | `Bool` | Of een netwerkverzoek bezig is |
| `hasMore` | `Bool` | Of er meer pagina's met reacties bestaan |
| `blockingErrorMessage` | `String?` | Fout die voorkomt dat de UI werkt |
| `warningMessage` | `String?` | Niet-blokkerend waarschuwingsbericht |
| `isDemo` | `Bool` | Of het in demomodus draait |
| `commentsVisible` | `Bool` | Schakelaar voor zichtbaarheid van reacties |
| `toolbarEnabled` | `Bool` | Of de opmaakwerkbalk wordt weergegeven |

### Gepubliceerde eigenschappen van FastCommentsFeedSDK

| Eigenschap | Type | Beschrijving |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Momenteel geladen feed-berichten |
| `hasMore` | `Bool` | Of er meer pagina's bestaan |
| `currentUser` | `UserSessionInfo?` | Huidige geauthenticeerde gebruiker |
| `blockingErrorMessage` | `String?` | Blokkerend foutbericht |
| `isLoading` | `Bool` | Of een netwerkverzoek bezig is |
| `newPostsCount` | `Int` | Aantal nieuwe berichten sinds laatste laden |

### Reactieboom

De reactieboom is toegankelijk via `sdk.commentsTree`:

```swift
// Platte lijst van zichtbare knooppunten voor weergave
sdk.commentsTree.visibleNodes

// Zoek een reactie op aan de hand van ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---