המערכת של הפיד היא SDK נפרד (`FastCommentsFeedSDK`) עם תצוגה משלה.

### טעינה והצגת הפיד

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
                // הצג גליון שיתוף
            }
            .onUserClick { context, userInfo, source in
                // נווט לפרופיל המשתמש
            }
            .onMediaClick { mediaItem, index in
                // הצג מציג תמונה במסך מלא
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

תצוגת הפיד כוללת משיכה לרענון וגלילה אינסופית אוטומטית.
השתמש ב-`loadIfNeeded()` בעת חידוש מחזור חיי המסך כדי שפיד קיים או משוחזר לא יאופס חזרה לדף 1.

### יצירת פוסטים

השתמש ב-`FeedPostCreateView` להצגת טופס יצירת פוסט:

```swift
@State private var showCreatePost = false

// בגוף התצוגה שלך:
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

### תגובה לפוסטים

ה-SDK מטפל בתגובות עם עדכונים אופטימיסטיים:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// בדוק את מצב התגובה
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### פתיחת תגובות על פוסט

השתמש ב-`CommentsSheet` כדי להציג תגובות עבור פוסט בפיד. הוא יוצר מופע של `FastCommentsSDK` פנימית באמצעות הקונפיגורציה של ה-feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // טיפול בלחיצה על משתמש
    })
}
```

הערה: `FeedPost` חייב להתאים ל-`Identifiable` עבור `.sheet(item:)`. הוסף את ההרחבה הבאה:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### סינון פיד לפי תגיות

ממש את פרוטוקול `TagSupplier` כדי לסנן פוסטים בפיד לפי תגיות:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

החזר `nil` עבור פיד גלובלי ללא סינון.

### שמירה ושחזור מצב הפיד

שמור את מצב הדפדוף (pagination) בין אירועי מחזור חיי התצוגה:

```swift
let state = sdk.savePaginationState()
// מאוחר יותר...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

אם המסך שלך נעלם זמנית, תצוגת הפיד עוצרת עדכונים חיים אוטומטית וממשיכה אותם בהופעה חוזרת מבלי לנקות פוסטים שהוטענו. קרא ל-`sdk.cleanup()` רק כשתהיה באמת גמור עם מופע ה-SDK.

### מחיקת פוסטים

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```