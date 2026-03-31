Hem `FastCommentsSDK` hem de `FastCommentsFeedSDK`, `@Published` özelliklere sahip `ObservableObject` sınıflarıdır. Reaktif UI güncellemeleri için bunları SwiftUI görünümlerinizde gözlemleyebilirsiniz.

### FastCommentsSDK Yayınlanan Özellikleri

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Sunucudaki toplam yorum sayısı |
| `newRootCommentCount` | `Int` | Arabelleklenmiş yeni yorumlar ( `showLiveRightAway` false olduğunda ) |
| `currentUser` | `UserSessionInfo?` | Mevcut kimliği doğrulanmış kullanıcı |
| `isSiteAdmin` | `Bool` | Geçerli kullanıcının site yöneticisi olup olmadığı |
| `isClosed` | `Bool` | Yorum dizisinin kapalı olup olmadığı |
| `hasBillingIssue` | `Bool` | Bir faturalama sorunu olup olmadığı |
| `isLoading` | `Bool` | Bir ağ isteğinin devam etmekte olup olmadığı |
| `hasMore` | `Bool` | Daha fazla yorum sayfasının olup olmadığı |
| `blockingErrorMessage` | `String?` | UI'nın çalışmasını engelleyen hata |
| `warningMessage` | `String?` | Engelleyici olmayan uyarı mesajı |
| `isDemo` | `Bool` | Demo modunda çalışıp çalışmadığı |
| `commentsVisible` | `Bool` | Yorum görünürlüğünü açıp kapama |
| `toolbarEnabled` | `Bool` | Biçimlendirme araç çubuğunun gösterilip gösterilmediği |

### FastCommentsFeedSDK Yayınlanan Özellikleri

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Şu anda yüklü feed gönderileri |
| `hasMore` | `Bool` | Daha fazla sayfa olup olmadığı |
| `currentUser` | `UserSessionInfo?` | Mevcut kimliği doğrulanmış kullanıcı |
| `blockingErrorMessage` | `String?` | Engelleyici hata mesajı |
| `isLoading` | `Bool` | Bir ağ isteğinin devam etmekte olup olmadığı |
| `newPostsCount` | `Int` | Son yüklemeden bu yana yeni gönderi sayısı |

### Yorum Ağacı

Yorum ağacına `sdk.commentsTree` üzerinden erişilebilir:

```swift
// Flat list of visible nodes for rendering
sdk.commentsTree.visibleNodes

// Lookup a comment by ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---