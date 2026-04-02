`LiveChatView` παρέχει εμπειρία συνομιλίας σε πραγματικό χρόνο με αυτόματο κύλισμα, διαχωριστικά ημερομηνιών και συμπαγή διάταξη. Διαμορφώνει αυτόματα το SDK για ταξινόμηση από τα παλαιότερα προς τα νεότερα και άμεση ζωντανή εμφάνιση.

```swift
struct ChatView: View {
    @StateObject private var sdk: FastCommentsSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "chat-room-1",
            sso: ssoToken  // Συνιστάται SSO ώστε οι χρήστες να έχουν ονόματα
        )
        return FastCommentsSDK(config: config)
    }()

    var body: some View {
        LiveChatView(sdk: sdk)
            .onCommentPosted { comment in
                print("Sent: \(comment.commentHTML)")
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

`LiveChatView` υποστηρίζει τις ακόλουθες callbacks:

- `.onCommentPosted` -- εκτελείται όταν ο χρήστης στέλνει ένα μήνυμα
- `.onCommentDeleted` -- εκτελείται όταν ένα μήνυμα διαγράφεται
- `.onUserClick` -- εκτελείται όταν πατηθεί το όνομα ή το avatar ενός χρήστη

---
---