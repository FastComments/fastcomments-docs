Eklenti ayarları sayfası **Site Yönetimi > Eklentiler > Yerel eklentiler > FastComments** konumundadır. Mevcut seçenekler şunlardır:

#### Tenant ID

FastComments Tenant ID'niz. Bunu hesap ayarlarınız altında <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments kontrol panelinde</a> bulun.

#### API Secret

Güvenli SSO modu için gerekli API Secret anahtarınız. Bunu <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a> altında bulun.

#### SSO Mode

Kullanıcıların nasıl kimlik doğrulanacağını seçin. Her seçenek hakkında ayrıntılar için [SSO Modları](#moodle-sso-modes) bölümüne bakın.

- **Secure** (önerilir) - sunucu tarafı HMAC-SHA256 ile imzalanmış kimlik doğrulama
- **Simple** - imzasız, istemci tarafı kullanıcı verisi
- **None** - anonim yorum, Moodle giriş entegrasyonu yok

#### Page Contexts

Yorumların nerede görüneceğini kontrol eder:

- **Course pages** - kurs ana sayfalarında yorumlar
- **Module/activity pages** - bireysel etkinlik ve kaynak sayfalarında yorumlar
- **Both** - tüm sayfa türlerinde yorumlar

#### Commenting Style

Yorum deneyimini seçin. Her modun ekran görüntüleri için [Yorum Stilleri](#moodle-commenting-styles) bölümüne bakın.

- **Comments** - sayfa içeriğinin altında standart dallanmış yorum bileşeni
- **Collab Chat** - varlık göstergeleriyle satır içi metin seçimi tartışmaları
- **Both** - hem yorumlar hem de collab chat birlikte aktif

#### CDN URL

FastComments CDN URL'si. Varsayılan olarak `https://cdn.fastcomments.com` kullanılır. Verileriniz AB bölgesinde barındırılıyorsa bunu AB CDN URL'si ile değiştirin.