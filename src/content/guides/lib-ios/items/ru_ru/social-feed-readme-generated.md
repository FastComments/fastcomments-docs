Система ленты — отдельный SDK (`FastCommentsFeedSDK`) со своим представлением.

### Загрузка и отображение ленты

```swift
struct FeedPage: View {
    @StateObject private var sdk: FastCommentsFeedSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-feed",
            sso: ssoToken
        )
        return FastCommentsFeedSDK(config: config)
    }()

    @State private var commentsPost: FeedPost?

    var body: some View {
        FastCommentsFeedView(sdk: sdk)
            .onPostSelected { post in
                commentsPost = post
            }
            .onCommentsRequested { post in
                commentsPost = post
            }
            .onSharePost { post in
                // Открыть окно общего доступа
            }
            .onUserClick { context, userInfo, source in
                // Перейти к профилю пользователя
            }
            .onMediaClick { mediaItem, index in
                // Показать полноэкранный просмотрщик изображений
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Представление ленты автоматически поддерживает pull-to-refresh и бесконечную прокрутку.

### Создание постов

Используйте `FeedPostCreateView` для отображения формы создания поста:

```swift
@State private var showCreatePost = false

// В теле вашего представления:
.sheet(isPresented: $showCreatePost) {
    FeedPostCreateView(
        sdk: sdk,
        onPostCreated: { post in
            showCreatePost = false
            Task { try? await sdk.refresh() }
        },
        onCancelled: {
            showCreatePost = false
        }
    )
}
```

### Реакции на посты

SDK обрабатывает реакции с оптимистичными обновлениями:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Открытие комментариев для поста

Используйте `CommentsSheet` для отображения комментариев к посту ленты. Внутри он создаёт экземпляр `FastCommentsSDK`, используя конфигурацию feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обработать клик по пользователю
    })
}
```

Примечание: `FeedPost` должен соответствовать `Identifiable` для использования `.sheet(item:)`. Добавьте это расширение:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Фильтрация ленты по тегам

Реализуйте протокол `TagSupplier`, чтобы фильтровать посты ленты по тегам:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Возвращайте `nil` для нефильтрованной глобальной ленты.

### Сохранение и восстановление состояния ленты

Сохраняйте состояние пагинации при событиях жизненного цикла представления:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Удаление постов

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```