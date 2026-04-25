Tüm ayarlar `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`) altında bulunur.

## Gereksinimler

- **Tenant ID** - FastComments Tenant ID'niz. Bunu [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) altında bulun.
- **API Secret** - Secure SSO, webhook doğrulaması ve sayfa eşitleme için gereklidir. [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) altında bulunur.

## Yorum Stili

Sitenizde insanların nasıl konuşmasını istediğinize uygun widget'i seçin.

- **Live Comments** - Gerçek zamanlı dallanmış yorumlar.
- **Streaming Chat** - Canlı sohbet arayüzü; etkinlikler ve canlı yayınlar için uygun.
- **Collab Chat** - Ana içerik alanında metin seçimine dayalı açıklama. Ziyaretçiler metni vurgular ve bağlam içinde tartışma başlatır.
- **Collab Chat + Comments** - Aynı sayfada hem collab chat hem de standart yorumlar.

## SSO Modu

- **None** - SSO yok. Kullanıcılar misafir olarak yorum yapar veya bir FastComments hesabı oluşturur.
- **Simple** - Drupal kullanıcı bilgilerini (isim, e-posta, avatar) sunucu tarafı doğrulama olmadan FastComments'a iletir.
- **Secure** - Drupal kullanıcılarını FastComments ile doğrulamak için HMAC-SHA256 kullanır. Bir API Secret yapılandırıldığında önerilir.

Detaylar için `Single Sign-On (SSO)` bölümüne bakın.

## Diğer Ayarlar

- **CDN URL** - Varsayılan olarak `https://cdn.fastcomments.com`.
- **Site URL** - Varsayılan olarak `https://fastcomments.com`.
- **Email notifications** - Yeni bir yorum içeriğe eklendiğinde içerik yazarına e-posta gönderir.

EU veri yerleşimi için `EU Data Residency` bölümüne bakın.