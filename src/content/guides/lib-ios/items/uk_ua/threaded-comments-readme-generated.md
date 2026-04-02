### Базове використання

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

### Стилі голосування

За замовчуванням стиль голосування відображає стрілки вгору/вниз. Передайте `._1` для стилю з сердечком:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | Зовнішній вигляд |
|-------|------------------|
| `._0` | Кнопки зі стрілками вгору/вниз із загальним лічильником |
| `._1` | Одна кнопка-сердечко з лічильником |

### Обробники подій

Використовуйте колбеки у вигляді модифікаторів для обробки взаємодій користувача:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source — це .name або .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Застосування теми

Передайте тему через середовище SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Або встановіть її безпосередньо в SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Напрям сортування

```swift
sdk.defaultSortDirection = .nf  // Найновіші першими (за замовчуванням)
sdk.defaultSortDirection = .of  // Найстаріші першими
sdk.defaultSortDirection = .mr  // Найрелевантніші
```

---
---