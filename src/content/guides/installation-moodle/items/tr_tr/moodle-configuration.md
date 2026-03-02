Eklenti ayarları sayfası şu konumdadır **Site Yönetimi > Eklentiler > Yerel eklentiler > FastComments**. Mevcut seçenekler:

#### Tenant ID

FastComments Tenant ID'niz. Bunu <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments kontrol paneli</a> içinde hesap ayarlarınızda bulabilirsiniz.

#### API Secret

Güvenli SSO modu için gereken API Secret anahtarınız. Bunu <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Hesabım > API Secret</a> altında bulun.

#### SSO Mode

Kullanıcıların nasıl kimlik doğrulanacağını seçin. Her seçenek hakkında ayrıntılar için [SSO Modları](#moodle-sso-modes) bölümüne bakın.

- **Secure** (önerilir) - sunucu tarafı HMAC-SHA256 ile imzalanmış kimlik doğrulama
- **Simple** - imzasız istemci tarafı kullanıcı verisi
- **None** - anonim yorum, Moodle oturum açma entegrasyonu yok

#### Page Contexts

Yorumların nerede görüneceğini kontrol eder:

- **Course pages** - kurs ana sayfalarında yorumlar
- **Module/activity pages** - bireysel etkinlik ve kaynak sayfalarında yorumlar
- **Both** - tüm sayfa türlerinde yorumlar

#### Commenting Style

Yorum deneyimini seçin. Her modun ekran görüntüleri için [Yorum Stilleri](#moodle-commenting-styles) bölümüne bakın.

- **Comments** - sayfa içeriğinin altında standart dallanmış yorum bileşeni
- **Collab Chat** - varlık göstergeli satır içi metin seçimi tartışmaları
- **Both** - yorumlar ve collab chat birlikte aktif

#### CDN URL

FastComments CDN URL'si. Varsayılan olarak `https://cdn.fastcomments.com` kullanılır. Verileriniz AB bölgesinde barındırılıyorsa bunu AB CDN URL'si ile değiştirin.