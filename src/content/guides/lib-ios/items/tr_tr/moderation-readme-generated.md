### Tüm Kullanıcılar İçin Mevcut İşlemler

- **Flag/Unflag** -- bir yorumu gözden geçirme için raporla

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- bir kullanıcıdan gelen tüm yorumları gizle (görülere göre)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Yalnızca Yöneticiler İçin İşlemler

- **Pin/Unpin** -- bir yorumu dizinin en üstüne sabitle

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- bir yoruma yeni yanıtları engelle ve kilit açılana kadar düzenlemeleri ve silmeleri engelle (herkese, moderatörler dahil, uygulanır)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Tüm denetleme işlemleri, UI'deki yorum bağlam menüsü aracılığıyla da kullanılabilir. Yönetici işlemleri yalnızca geçerli kullanıcı bir site yöneticisi olduğunda görünür (SSO `isAdmin` bayrağı veya kontrol paneli yapılandırması ile ayarlanır).