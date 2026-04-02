Både `FastCommentsSDK` og `FastCommentsFeedSDK` er `ObservableObject`-klasser med `@Published`-egenskaber. Du kan observere disse i dine SwiftUI-visninger for reaktive UI-opdateringer.

### FastCommentsSDK Publicerede Egenskaber

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Samlet antal kommentarer på serveren |
| `newRootCommentCount` | `Int` | Bufferede nye kommentarer (når `showLiveRightAway` is false) |
| `currentUser` | `UserSessionInfo?` | Nuværende autentificerede bruger |
| `isSiteAdmin` | `Bool` | Om den aktuelle bruger er siteadministrator |
| `isClosed` | `Bool` | Om kommentertråden er lukket |
| `hasBillingIssue` | `Bool` | Om der er et betalingsproblem |
| `isLoading` | `Bool` | Om en netværksanmodning er i gang |
| `hasMore` | `Bool` | Om der findes flere sider med kommentarer |
| `blockingErrorMessage` | `String?` | Fejl, der forhindrer brugergrænsefladen i at fungere |
| `warningMessage` | `String?` | Ikke-blokerende advarselsmeddelelse |
| `isDemo` | `Bool` | Om der køres i demotilstand |
| `commentsVisible` | `Bool` | Om kommentarer er synlige |
| `toolbarEnabled` | `Bool` | Om formateringsværktøjslinjen vises |

### FastCommentsFeedSDK Publicerede Egenskaber

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Aktuelt indlæste feedindlæg |
| `hasMore` | `Bool` | Om der findes flere sider |
| `currentUser` | `UserSessionInfo?` | Nuværende autentificerede bruger |
| `blockingErrorMessage` | `String?` | Blokerende fejlmeddelelse |
| `isLoading` | `Bool` | Om en netværksanmodning er i gang |
| `newPostsCount` | `Int` | Antal nye indlæg siden sidste indlæsning |

### Kommentartræ

Kommentartræet kan tilgås via `sdk.commentsTree`:

```swift
// Flad liste over synlige noder til gengivelse
sdk.commentsTree.visibleNodes

// Find en kommentar efter ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---