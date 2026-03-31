### Edit

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Sunucu HTML'yi yeniden render eder. Yerel yorum otomatik olarak güncellenir.

### Delete

```swift
try await sdk.deleteComment(commentId: commentId)
```

Bir yorumu silmek, yerel ağaçtaki tüm alt öğelerini de kaldırır.

Her iki işlem de, geçerli kullanıcı yorumun yazarı (veya site yöneticisi) olduğunda, arayüzdeki yorum bağlam menüsü aracılığıyla kullanılabilir.

---
---