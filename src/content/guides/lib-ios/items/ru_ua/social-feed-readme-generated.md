---
Система ленты — отдельный SDK (`FastCommentsFeedSDK`) с собственным видом.

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
                // Отобразить панель обмена
            }
            .onUserClick { context, userInfo, source in
                // Перейти к профилю пользователя
            }
            .onMediaClick { mediaItem, index in
                // Показать просмотрщик изображений на весь экран
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Просмотр ленты включает pull-to-refresh и бесконечную прокрутку автоматически.
Вызывайте `loadIfNeeded()` при повторном появлении экрана в жизненном цикле, чтобы существующая или восстановленная лента не сбрасывалась на страницу 1.

### Создание постов

Используйте `FeedPostCreateView` для отображения формы создания поста:

```swift
@State private var showCreatePost = false

// В теле вашего view:
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

### Открытие комментариев к посту

Используйте `CommentsSheet` для отображения комментариев к посту в ленте. Он создаёт экземпляр `FastCommentsSDK` внутри, используя конфиг feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обработать клик по пользователю
    })
}
```

Note: `FeedPost` must conform to `Identifiable` for `.sheet(item:)`. Add this extension:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Фильтрация ленты по тегам

Реализуйте протокол `TagSupplier` для фильтрации постов ленты по тегам:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Верните `nil` для нефильтрованной глобальной ленты.

### Сохранение и восстановление состояния ленты

Сохраняйте состояние пагинации при событиях жизненного цикла представления:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Если ваш экран временно исчезает, просмотр ленты приостанавливает живые обновления автоматически и возобновляет их при повторном появлении без очистки загруженных постов. Вызывайте `sdk.cleanup()` только тогда, когда вы действительно завершили работу с экземпляром SDK.

### Удаление постов

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---