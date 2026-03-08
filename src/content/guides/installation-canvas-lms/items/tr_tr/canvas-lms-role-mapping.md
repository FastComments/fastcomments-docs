Canvas rolleri LTI başlatması sırasında otomatik olarak FastComments rollerine eşlenir. Manuel yapılandırma gerekmez.

#### Rol Eşlemesi

| Canvas Rolü | FastComments Rolü | İzinler |
|---|---|---|
| **Yönetici** | Admin | Tam hesap erişimi, tüm yorumları ve ayarları yönetme |
| **Eğitmen** | Moderator | Yorumları düzenleme ve silme, konuları sabitleme, tartışmaları yönetme |
| **Öğrenci** | Commenter | Yorum gönderme, yanıt verme, oy kullanma ve bahsetmeleri kullanma |

#### Nasıl Çalışır

Bir kullanıcı FastComments'i Canvas üzerinden başlattığında, LTI 1.3 protokolü onların Canvas rolünü içerir. FastComments bu rolü okur ve uygun izinleri otomatik olarak atar.

Bir kullanıcının birden fazla rolü varsa (örn. hem Eğitmen hem de Admin olan biri), en yüksek ayrıcalığa sahip rol kullanılır.