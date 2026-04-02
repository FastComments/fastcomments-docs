### Sayfa Boyutu

```swift
// Yorumlar: varsayılan 30
sdk.pageSize = 50

// Besleme: varsayılan 10
feedSDK.pageSize = 20
```

### Daha Fazla Yorum Yükleme

Arayüz sayfalama kontrollerini otomatik olarak gösterir. Ayrıca sayfalamayı programlı olarak da tetikleyebilirsiniz:

```swift
// Sonraki sayfayı yükle
try await sdk.loadMore()

// Kalan tümünü yükle (performans nedeniyle >2000 yorum varsa devre dışı bırakılır)
try await sdk.loadAll()

// Durumu kontrol et
sdk.hasMore            // Daha fazla sayfa var mı
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Alt Yorum Sayfalandırma

İç içe yanıtlar tembel olarak yüklenir. Bir kullanıcı bir diziyi genişlettiğinde, ilk 5 alt yorum yüklenir. Daha fazla varsa "daha fazla yanıtı yükle" kontrolü görünür. Bu, arayüz tarafından otomatik olarak yönetilir.

---
---