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

### Στυλ Ψήφων

Το προεπιλεγμένο στυλ ψήφου εμφανίζει βέλη πάνω/κάτω. Χρησιμοποιήστε `._1` για ψήφους τύπου καρδιάς:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Στυλ | Εμφάνιση |
|-------|------------|
| `._0` | Κουμπιά βέλους πάνω/κάτω με καθαρό σύνολο |
| `._1` | Ενιαίο κουμπί καρδιάς με μέτρηση |

### Κλήσεις επιστροφής συμβάντων

Χρησιμοποιήστε κλήσεις επιστροφής σε μορφή modifier για να χειριστείτε τις αλληλεπιδράσεις του χρήστη:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source είναι .name ή .avatar
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
sdk.defaultSortDirection = .nf  // Πρώτα τα πιο πρόσφατα (προεπιλογή)
sdk.defaultSortDirection = .of  // Πρώτα τα παλαιότερα
sdk.defaultSortDirection = .mr  // Πιο σχετικά
```