### Основно използване

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

### Стилове гласуване

По подразбиране стилът за гласуване показва стрелки нагоре/надолу. Използвайте `._1` за стил гласуване със сърце:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Стил | Външен вид |
|-------|------------|
| `._0` | Бутони със стрелки нагоре/надолу с нетен брой |
| `._1` | Един бутон със сърце и брой |

### Обратни повиквания за събития

Използвайте модификаторни callback-и за обработка на потребителските взаимодействия:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source е .name или .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Прилагане на тема

Подайте тема чрез SwiftUI средата:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Или я задайте директно в SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Посока на сортиране

```swift
sdk.defaultSortDirection = .nf  // Най-нови първи (по подразбиране)
sdk.defaultSortDirection = .of  // Най-стари първи
sdk.defaultSortDirection = .mr  // Най-релевантни
```

---
---