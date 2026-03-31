sdk.load() çağrıldıktan sonra, SDK yapılandırılmış `urlId` için WebSocket etkinliklerine otomatik olarak abone olur. Aşağıdaki etkinlikler işlenir:

- Yeni yorumlar, düzenlemeler ve silinmeler
- Oylar (yeni ve kaldırılan)
- Sabitleme, kilitleme, işaretleme ve engelleme durum değişiklikleri
- Kullanıcı varlığı (katılma/ayrılma)
- İleti dizisi açma/kapatma
- Rozet verilmesi
- Sunucu yapılandırması güncellemeleri

### Canlı Görünümü Kontrol Etme

Varsayılan olarak, diğer kullanıcılardan gelen yeni yorumlar hemen görünür:

```swift
sdk.showLiveRightAway = true   // Varsayılan: anında göster
```

Bunu `false` olarak ayarlayın, yeni yorumları "N yeni yorum" düğmesinin arkasında tamponlayarak kullanıcının bunları ne zaman göstereceğini seçmesine izin verir:

```swift
sdk.showLiveRightAway = false
```

### Kullanıcı Varlığı

Sunucu varlık takibini etkinleştirdiğinde, çevrimiçi/çevrimdışı göstergeleri otomatik olarak kullanıcı avatarlarında görünür. İstemci tarafında ek bir yapılandırma gerekmez.

---
---