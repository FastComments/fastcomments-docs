Eklenti, Moodle kullanıcı hesaplarını FastComments ile entegre etmek için üç SSO modu destekler.

#### Güvenli SSO (Önerilen)

Kullanıcı verileri, API Secret'ınızla HMAC-SHA256 kullanılarak sunucu tarafında imzalanır. Bu en güvenli seçenektir ve üretim kullanımı için önerilir.

Güvenli SSO ile:

- Kullanıcı adları, e-postalar ve avatarlar FastComments'e güvenli bir şekilde iletilir.
- Moodle site yöneticileri otomatik olarak FastComments moderatörleri olarak senkronize edilir.
- Kullanıcılar diğer hesapları taklit edemez.
- Eklenti ayarlarında **API Secret**'ın yapılandırılmasını gerektirir.

#### Basit SSO

Kullanıcı verileri (isim, e-posta, avatar) kriptografik imza olmadan istemci tarafında gönderilir. API Secret gerektirmediği için kurulumu daha kolaydır, ancak kullanıcı verileri sunucu tarafında doğrulanmadığı için daha az güvenlidir.

#### Hiçbiri

SSO entegrasyonu yok. Kullanıcılar anonim olarak yorum yapar veya FastComments'e ayrı olarak giriş yapmak zorundadır. Moodle kullanıcı hesaplarını yorumlarla ilişkilendirmek istemiyorsanız bunu kullanın.