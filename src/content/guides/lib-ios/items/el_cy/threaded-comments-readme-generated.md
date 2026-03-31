### Βασική Χρήση

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

### Στυλ Ψηφοφορίας

Το προεπιλεγμένο στυλ ψήφου εμφανίζει βέλη πάνω/κάτω. Περάστε `._1` για ψήφους με σχήμα καρδιάς:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | Εμφάνιση |
|-------|------------|
| `._0` | Κουμπιά βελών πάνω/κάτω με καθαρό πλήθος |
| `._1` | Μονό κουμπί καρδιάς με πλήθος |

### Κλήσεις callback

Χρησιμοποιήστε callbacks τύπου modifier για να χειριστείτε τις αλληλεπιδράσεις του χρήστη:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // το source είναι .name ή .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Εφαρμογή Θέματος

Περάστε ένα θέμα μέσω του περιβάλλοντος SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Ή ορίστε το απευθείας στο SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Κατεύθυνση Ταξινόμησης

```swift
sdk.defaultSortDirection = .nf  // Νεότερα πρώτα (προεπιλογή)
sdk.defaultSortDirection = .of  // Παλαιότερα πρώτα
sdk.defaultSortDirection = .mr  // Πιο σχετικό
```

---
---