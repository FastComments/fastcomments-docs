All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## Gerekli

- **Tenant ID** - FastComments Kiracı Kimliğiniz. Bunu [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) altında bulun ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Güvenli SSO, webhook doğrulaması ve sayfa senkronizasyonu için gereklidir. [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) altında bulunur ([EU](https://eu.fastcomments.com/auth/my-account/api)).

## API Gizli Anahtarını Saklama

Modül, API Gizli Anahtarını değer içeren bu kaynaklardan ilkinden okur:

1. **Key module** - [Key](https://www.drupal.org/project/key) kurun, bir *Authentication* anahtarı oluşturun ve ayar formundaki **API Secret key** altında seçin. Anahtar bir ortam değişkeni veya web kökünün dışındaki bir dosya ile desteklenebilir.
2. **settings.php** - `$config['fastcomments.settings']['api_secret'] = 'your-secret';` satırını `settings.php` veya `settings.local.php` dosyasına ekleyin. Gizli anahtar hiçbir zaman dışa aktarılan yapılandırmaya girmez.
3. **The settings form** - **API Secret** alanına girin. Bu, değeri `fastcomments.settings` içinde saklar, böylece yapılandırma dışa aktarımlarına dahil olur. Drupal durum raporu, bu kaynağın aktif olduğunda uyarı verir.

Form, hangi kaynağın aktif olduğunu gösterir. Alanı boş bırakarak saklanan gizli anahtar korunur. Gizli anahtarı Key modülüne veya `settings.php`'ye taşıdıktan sonra silmek için **Yapılandırmada saklanan API Gizli Anahtarını Kaldır** kutusunu işaretleyin.

## Yorumlama Stili

Sitenizde insanların nasıl konuşmasını istediğinize uygun widget'ı seçin.

- **Live Comments** - Gerçek zamanlı dallı yorumlar.
- **Streaming Chat** - Canlı sohbet arayüzü, etkinlikler ve canlı yayınlar için uygundur.
- **Collab Chat** - Ana içerik alanında metin seçimiyle açıklama. Ziyaretçiler metni vurgular ve bağlam içinde bir tartışma başlatır.
- **Collab Chat + Comments** - Aynı sayfada hem işbirliği sohbeti hem de standart yorumlar.

## SSO Modu

- **None** - SSO yok. Kullanıcılar misafir olarak yorum yapar veya bir FastComments hesabı oluşturur.
- **Simple** - Drupal kullanıcı bilgilerini (ad, e-posta, avatar) FastComments'a sunucu tarafı doğrulama olmadan gönderir.
- **Secure** - Drupal kullanıcılarını FastComments ile doğrulamak için HMAC‑SHA256 kullanır. API Gizli Anahtarı yapılandırılmışsa önerilir.

Ayrıntılar için `Single Sign-On (SSO)` bölümüne bakın.

## Diğer Ayarlar

- **CDN URL** - Varsayılan olarak `https://cdn.fastcomments.com` adresidir.
- **Site URL** - Varsayılan olarak `https://fastcomments.com` adresidir.
- **Email notifications** - Yeni bir yorum içeriklerine gönderildiğinde içerik yazarına e-posta gönderilir.

AB veri konumlandırması için `EU Data Residency` bölümüne bakın.

---