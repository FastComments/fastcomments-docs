### Δράσεις Διαθέσιμες σε Όλους τους Χρήστες

- **Σήμανση/Αποσήμανση** -- αναφορά ενός σχολίου για έλεγχο

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Αποκλεισμός/Κατάργηση Αποκλεισμού** -- απόκρυψη όλων των σχολίων από έναν χρήστη (ανά θεατή)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Δράσεις Μόνο για Διαχειριστές

- **Καρφίτσωμα/Αφαίρεση καρφιτσώματος** -- καρφίτσωμα ενός σχολίου στην κορυφή του νήματος

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Κλείδωμα/Ξεκλείδωμα** -- αποτροπή νέων απαντήσεων σε ένα σχόλιο

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

All moderation actions are also available through the comment context menu in the UI. Admin actions only appear when the current user is a site admin (set via SSO `isAdmin` flag or dashboard configuration).