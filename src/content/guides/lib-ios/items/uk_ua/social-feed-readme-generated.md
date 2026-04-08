Система стрічки — це окремий SDK (`FastCommentsFeedSDK`) з власним поданням.

### Завантаження та відображення стрічки

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
                // Відкрити діалог поширення
            }
            .onUserClick { context, userInfo, source in
                // Перейти до профілю користувача
            }
            .onMediaClick { mediaItem, index in
                // Показати переглядач зображення на весь екран
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Подання стрічки автоматично підтримує pull-to-refresh та нескінченну прокрутку.
Використовуйте `loadIfNeeded()` при поверненні екрана в життєвому циклі, щоб існуюча або відновлена стрічка не скидалася назад на сторінку 1.

### Створення постів

Використовуйте `FeedPostCreateView` щоб відобразити форму створення поста:

```swift
@State private var showCreatePost = false

// У тілі вашого подання:
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

### Реакції на пости

SDK оброблює реакції з оптимістичним оновленням:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Відкриття коментарів для поста

Використовуйте `CommentsSheet` для відображення коментарів для поста у стрічці. Він внутрішньо створює екземпляр `FastCommentsSDK`, використовуючи конфігурацію SDK стрічки:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обробити клік користувача
    })
}
```

Примітка: `FeedPost` повинен відповідати протоколу `Identifiable` для `.sheet(item:)`. Додайте це розширення:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Фільтрація стрічки за тегами

Реалізуйте протокол `TagSupplier`, щоб фільтрувати пости стрічки за тегами:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Поверніть `nil` для нефільтрованої глобальної стрічки.

### Збереження та відновлення стану стрічки

Зберігайте стан пагінації під час подій життєвого циклу подання:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Якщо ваш екран тимчасово зникає, подання стрічки автоматично призупиняє live-оновлення і відновлює їх при повторній появі без очищення завантажених постів. Викликайте `sdk.cleanup()` тільки коли ви дійсно завершили роботу з екземпляром SDK.

### Видалення постів

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```