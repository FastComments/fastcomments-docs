Система стрічки — це окремий SDK (`FastCommentsFeedSDK`) зі своїм власним виглядом.

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
                // Показати діалог спільного доступу
            }
            .onUserClick { context, userInfo, source in
                // Перейти до профілю користувача
            }
            .onMediaClick { mediaItem, index in
                // Показати переглядач зображення на весь екран
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Вигляд стрічки автоматично підтримує потягування для оновлення та безкінечну прокрутку.

### Створення дописів

Використовуйте `FeedPostCreateView` для відображення форми створення допису:

```swift
@State private var showCreatePost = false

// У тілі вашого вигляду:
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

### Реакції на дописи

SDK обробляє реакції з оптимістичними оновленнями:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Перевірити стан реакції
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Відкриття коментарів до допису

Використовуйте `CommentsSheet` для відображення коментарів до допису стрічки. Він створює екземпляр `FastCommentsSDK` всередині, використовуючи конфігурацію feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обробити натискання на користувача
    })
}
```

Примітка: `FeedPost` має відповідати протоколу `Identifiable` для `.sheet(item:)`. Додайте це розширення:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Фільтрація стрічки за тегами

Реалізуйте протокол `TagSupplier`, щоб фільтрувати дописи стрічки за тегами:

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

Збережіть стан пагінації між подіями життєвого циклу вигляду:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Видалення дописів

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```