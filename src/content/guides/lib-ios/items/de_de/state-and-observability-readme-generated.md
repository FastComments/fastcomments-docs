Sowohl `FastCommentsSDK` als auch `FastCommentsFeedSDK` sind `ObservableObject`-Klassen mit `@Published`-Eigenschaften. Sie können diese in Ihren SwiftUI-Ansichten beobachten, um reaktive UI-Aktualisierungen zu erhalten.

### FastCommentsSDK veröffentlichte Eigenschaften

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Gesamtanzahl der Kommentare auf dem Server |
| `newRootCommentCount` | `Int` | Zwischengespeicherte neue Kommentare (wenn `showLiveRightAway` false ist) |
| `currentUser` | `UserSessionInfo?` | Aktuell authentifizierter Benutzer |
| `isSiteAdmin` | `Bool` | Ob der aktuelle Benutzer ein Seitenadministrator ist |
| `isClosed` | `Bool` | Ob der Kommentarthread geschlossen ist |
| `hasBillingIssue` | `Bool` | Ob ein Abrechnungsproblem vorliegt |
| `isLoading` | `Bool` | Ob eine Netzwerk-Anfrage ausgeführt wird |
| `hasMore` | `Bool` | Ob weitere Seiten mit Kommentaren existieren |
| `blockingErrorMessage` | `String?` | Fehler, der die Funktionsfähigkeit der UI verhindert |
| `warningMessage` | `String?` | Nicht blockierende Warnmeldung |
| `isDemo` | `Bool` | Ob im Demo-Modus ausgeführt wird |
| `commentsVisible` | `Bool` | Schalter für die Sichtbarkeit der Kommentare |
| `toolbarEnabled` | `Bool` | Ob die Formatierungs-Symbolleiste angezeigt wird |

### FastCommentsFeedSDK veröffentlichte Eigenschaften

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Derzeit geladene Feed-Beiträge |
| `hasMore` | `Bool` | Ob weitere Seiten existieren |
| `currentUser` | `UserSessionInfo?` | Aktuell authentifizierter Benutzer |
| `blockingErrorMessage` | `String?` | Blockierende Fehlermeldung |
| `isLoading` | `Bool` | Ob eine Netzwerk-Anfrage ausgeführt wird |
| `newPostsCount` | `Int` | Anzahl neuer Beiträge seit dem letzten Laden |

### Kommentarbaum

Der Kommentarbaum ist über `sdk.commentsTree` zugänglich:

```swift
// Flache Liste sichtbarer Knoten zur Darstellung
sdk.commentsTree.visibleNodes

// Kommentar per ID nachschlagen
sdk.commentsTree.commentsById["comment-id"]
```

---
---