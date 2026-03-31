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
| `._0` | Дугмад са стрелицама горе/доле са нето резултатом |
| `._1` | Једно дугме у облику срца са бројем |

### Повратни позиви догађаја

Користите повратне позиве у облику модификатора за обраду интеракција са корисником:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source је .name или .avatar
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

Или је подесите директно у SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Смер сортирања

```swift
sdk.defaultSortDirection = .nf  // Најновије прво (подразумевано)
sdk.defaultSortDirection = .of  // Најстарије прво
sdk.defaultSortDirection = .mr  // Најрелевантније
```

---
---