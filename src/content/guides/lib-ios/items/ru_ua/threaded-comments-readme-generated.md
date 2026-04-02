### Базовое использование

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

### Стили голосования

Стиль голосования по умолчанию показывает стрелки вверх/вниз. Передайте `._1` для голосования в виде сердца:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Стиль | Внешний вид |
|-------|------------|
| `._0` | Кнопки со стрелками вверх/вниз с общим счётом |
| `._1` | Одна кнопка в виде сердца с показом количества |

### Обработчики событий

Используйте колбэки в стиле модификаторов для обработки взаимодействий пользователя:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source — это .name или .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Применение темы

Передайте тему через окружение SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Или задайте её напрямую в SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Направление сортировки

```swift
sdk.defaultSortDirection = .nf  // Сначала новые (по умолчанию)
sdk.defaultSortDirection = .of  // Сначала старые
sdk.defaultSortDirection = .mr  // Наиболее релевантные
```

---
---