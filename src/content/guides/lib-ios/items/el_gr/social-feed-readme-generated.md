Το σύστημα ροής είναι ένα ξεχωριστό SDK (`FastCommentsFeedSDK`) με τη δική του προβολή.

### Φόρτωση και Εμφάνιση της Ροής

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
                // Εμφάνιση φύλλου κοινής χρήσης
            }
            .onUserClick { context, userInfo, source in
                // Πλοήγηση στο προφίλ χρήστη
            }
            .onMediaClick { mediaItem, index in
                // Εμφάνιση προβολέα εικόνας πλήρους οθόνης
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Η προβολή της ροής περιλαμβάνει αυτόματα pull-to-refresh και infinite scroll.
Χρησιμοποιήστε `loadIfNeeded()` για επανείσοδο στον κύκλο ζωής της οθόνης ώστε μια υπάρχουσα ή αποκαταστημένη ροή να μην επανεκκινηθεί στην σελίδα 1.

### Δημιουργία Αναρτήσεων

Χρησιμοποιήστε το `FeedPostCreateView` για να εμφανίσετε μια φόρμα δημιουργίας ανάρτησης:

```swift
@State private var showCreatePost = false

// Στο σώμα της προβολής σας:
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

### Αντίδραση σε Αναρτήσεις

Το SDK χειρίζεται τις αντιδράσεις με optimistic updates:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Άνοιγμα Σχολίων για μια Ανάρτηση

Χρησιμοποιήστε το `CommentsSheet` για να εμφανίσετε σχόλια για μια ανάρτηση στη ροή. Δημιουργεί μια εσωτερική παρουσία `FastCommentsSDK` χρησιμοποιώντας το config του feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Διαχείριση κλικ του χρήστη
    })
}
```

Σημείωση: `FeedPost` πρέπει να συμμορφώνεται με το `Identifiable` για `.sheet(item:)`. Προσθέστε αυτήν την επέκταση:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Φιλτράρισμα Ροής με Ετικέτες

Υλοποιήστε το πρωτόκολλο `TagSupplier` για να φιλτράρετε τις αναρτήσεις της ροής κατά ετικέτες:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Επιστρέψτε `nil` για μια μη φιλτραρισμένη παγκόσμια ροή.

### Αποθήκευση και Επαναφορά Κατάστασης Ροής

Διατηρήστε την κατάσταση σελιδοποίησης κατά τα γεγονότα του κύκλου ζωής της προβολής:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Αν η οθόνη σας εξαφανιστεί προσωρινά, η προβολή της ροής παγώνει αυτόματα τις ενημερώσεις σε πραγματικό χρόνο και τις επαναφέρει όταν εμφανιστεί ξανά χωρίς να καθαρίσει τις φορτωμένες αναρτήσεις. Καλείτε `sdk.cleanup()` μόνο όταν τελείως ολοκληρώσετε με το instance του SDK.

### Διαγραφή Αναρτήσεων

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```