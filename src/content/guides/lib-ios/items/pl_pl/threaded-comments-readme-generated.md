### Podstawowe użycie

```swift
struct CommentsPage: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "article-42",
            url: "https://example.com/article/42",
            pageTitle: "Article Title"
        )
    )

    var body: some View {
        FastCommentsView(sdk: sdk)
            .task {
                try? await sdk.load()
            }
    }
}
```

### Style głosowania

Domyślny styl głosowania pokazuje strzałki w górę/w dół. Przekaż `._1` dla głosów w stylu serca:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Styl | Wygląd |
|-------|------------|
| `._0` | Przyciski strzałek góra/dół z wynikiem netto |
| `._1` | Pojedynczy przycisk serca z licznikiem |

### Wywołania zwrotne zdarzeń

Użyj wywołań zwrotnych w stylu modyfikatorów, aby obsłużyć interakcje użytkownika:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // źródło to .name lub .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Zastosowanie motywu

Przekaż motyw przez środowisko SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Lub ustaw go bezpośrednio w SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Kierunek sortowania

```swift
sdk.defaultSortDirection = .nf  // Najnowsze pierwsze (domyślnie)
sdk.defaultSortDirection = .of  // Najstarsze pierwsze
sdk.defaultSortDirection = .mr  // Najbardziej istotne
```

---
---