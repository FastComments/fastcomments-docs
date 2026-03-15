Şuraya gidin: **Administration > Configuration > Content > FastComments** (`/admin/config/content/fastcomments`).

### Ayarlar

- **Tenant ID** (gerekli) - FastComments Tenant ID'niz. Bunu [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) altında bulun.
- **API Secret** - Secure SSO, webhook doğrulaması ve sayfa senkronizasyonu için gereklidir. Bunu [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)) altında bulun.
- **SSO Mode** - Single Sign-On entegrasyonu:
  - **None** - SSO yok; kullanıcılar misafir olarak yorum yapar veya FastComments hesapları oluşturur.
  - **Simple** - Sunucu tarafı doğrulama olmadan Drupal kullanıcı bilgilerini (name, email, avatar) FastComments'a iletir.
  - **Secure** - Drupal kullanıcılarını FastComments ile güvenli şekilde kimlik doğrulamak için HMAC-SHA256 doğrulaması kullanır (önerilir).
- **Commenting Style** - Görüntülenecek widget türü:
  - **Live Comments** - Gerçek zamanlı dallanmış yorumlar.
  - **Streaming Chat** - Canlı sohbet arayüzü.
  - **Collab Chat** - Ana içerik alanında metin seçimine dayalı işbirlikçi açıklama.
  - **Collab Chat + Comments** - Hem collab chat hem de standart yorumlar.
- **CDN URL** - FastComments CDN URL'si (varsayılan: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments site URL'si (varsayılan: `https://fastcomments.com`).
- **Email notifications** - İçeriklerine yeni bir yorum yapıldığında içerik yazarlarına e-posta gönder.

### İçerik Türlerine Yorum Ekleme

FastComments alanını içerik türlerinize **Structure > Content types > [type] > Manage fields** üzerinden ekleyin. Alanın bir durum açma/kapama düğmesi ve her varlık için isteğe bağlı bir özel tanımlayıcısı vardır.

### AB Veri Yerleşimi

AB veri yerleşimi için, şu ayarları güncelleyin:
- **CDN URL**'yi `https://cdn-eu.fastcomments.com` olarak güncelleyin
- **Site URL**'yi `https://eu.fastcomments.com` olarak güncelleyin