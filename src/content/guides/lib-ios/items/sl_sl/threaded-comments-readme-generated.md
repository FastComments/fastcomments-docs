### Osnovna uporaba

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

### Slogi glasovanja

Privzeti slog glasovanja prikazuje puščici gor/dol. Uporabite `._1` za slog glasovanja s srcem:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Slog | Videz |
|-------|------------|
| `._0` | Gumbi s puščicami gor/dol z neto številom |
| `._1` | En sam gumb s srcem in prikazom števila |

### Povratni klici dogodkov

Uporabite povratne klice v obliki modifierjev za obravnavo interakcij uporabnika:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // vir je .name ali .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Uporaba teme

Posredujte temo prek SwiftUI okolja:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Ali jo nastavite neposredno na SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Smer sortiranja

```swift
sdk.defaultSortDirection = .nf  // Najnovejše najprej (privzeto)
sdk.defaultSortDirection = .of  // Najstarejše najprej
sdk.defaultSortDirection = .mr  // Najbolj relevantno
```

---
---