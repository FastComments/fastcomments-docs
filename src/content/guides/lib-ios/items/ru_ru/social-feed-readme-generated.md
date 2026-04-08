Система фида — отдельный SDK (`FastCommentsFeedSDK`) с собственным представлением.

### Загрузка и отображение фида

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
                // Показать окно обмена
            }
            .onUserClick { context, userInfo, source in
                // Перейти к профилю пользователя
            }
            .onMediaClick { mediaItem, index in
                // Показать полноэкранный просмотрщик изображений
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Просмотр фида автоматически поддерживает pull-to-refresh и бесконечную прокрутку.
Используйте `loadIfNeeded()` при повторном входе в жизненный цикл экрана, чтобы существующий или восстановленный фид не сбрасывался на страницу 1.

### Создание постов

Используйте `FeedPostCreateView` для отображения формы создания поста:

```swift
@State private var showCreatePost = false

// In your view body:
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

### Реакция на посты

SDK обрабатывает реакции с оптимистичными обновлениями:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Проверить состояние реакции
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Открытие комментариев к посту

Используйте `CommentsSheet` для отображения комментариев к посту фида. Он создаёт экземпляр `FastCommentsSDK` внутри, используя конфигурацию feed SDK:

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

### Фильтрация фида по тегам

Реализуйте протокол `TagSupplier` для фильтрации постов фида по тегам:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Верните `nil` для нефильтрованного глобального фида.

### Сохранение и восстановление состояния фида

Сохраняйте состояние пагинации при событиях жизненного цикла представления:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Если ваш экран временно исчезает, представление фида автоматически ставит на паузу live-обновления и возобновляет их при повторном появлении без очистки загруженных постов. Вызывайте `sdk.cleanup()` только когда вы действительно закончите с экземпляром SDK.

### Удаление постов

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```