### Основна употреба

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

### Стилови гласања

Подразумевани стил гласања приказује стрелице горе/доле. Проследите `._1` за стил гласања са срцем:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Стил | Изглед |
|-------|------------|
| `._0` | Дугмићи са стрелицама горе/доле и нето бројем |
| `._1` | Једно дугме у облику срца са бројем |

### Повратни позиви догађаја

Користите повратне позиве у облику модификатора да бисте обрадили корисничке интеракције:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source is .name or .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Примена теме

Проследите тему преко SwiftUI окружења:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Или је поставите директно на SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Смер сортирања

```swift
sdk.defaultSortDirection = .nf  // Newest first (default)
sdk.defaultSortDirection = .of  // Oldest first
sdk.defaultSortDirection = .mr  // Most relevant
```

---
---