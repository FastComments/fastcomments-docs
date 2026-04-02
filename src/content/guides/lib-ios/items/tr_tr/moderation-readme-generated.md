### Tüm Kullanıcılar İçin Kullanılabilir Eylemler

- **Bildir/Bildirimi Kaldır** -- bir yorumu inceleme için bildir

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Engelle/Engellemeyi Kaldır** -- bir kullanıcının tüm yorumlarını gizle (her görüntüleyici için)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Yalnızca Yönetici Eylemleri

- **Sabitle/Sabitlemeyi Kaldır** -- bir yorumu konu dizisinin en üstüne sabitle

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Kilitle/Kilidi Aç** -- yoruma yeni yanıtların eklenmesini engelle

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Tüm moderasyon eylemleri ayrıca kullanıcı arayüzündeki (UI) yorum bağlam menüsünden de kullanılabilir. Yönetici eylemleri yalnızca geçerli kullanıcı site yöneticisi olduğunda görünür (SSO ile ayarlanan `isAdmin` bayrağı veya yönetim paneli yapılandırması aracılığıyla).

---
---