Eklenti ayarları sayfası **Site Yönetimi > Eklentiler > Yerel eklentiler > FastComments** konumundadır. Kullanılabilir seçenekler şunlardır:

#### Tenant ID

FastComments Tenant ID'niz. Bunu <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments kontrol panelinde</a> hesap ayarlarınız altında bulabilirsiniz.

#### API Secret

Güvenli SSO modu için gerekli olan API Secret anahtarınız. Bunu <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">Hesabım > API Secret</a> konumunda bulabilirsiniz.

#### SSO Modu

Kullanıcıların nasıl kimlik doğrulayacağını seçin. Her seçenekle ilgili ayrıntılar için [SSO Modları](#moodle-sso-modes) bölümüne bakın.

- **Secure** (önerilen) - sunucu tarafında HMAC-SHA256 ile imzalanmış kimlik doğrulama
- **Simple** - imzasız, istemci tarafı kullanıcı verisi
- **None** - anonim yorum yapma, Moodle oturum açma entegrasyonu yok

#### Sayfa Bağlamları

Yorumların nerede görüneceğini kontrol eder:

- **Ders sayfaları** - ders ana sayfalarındaki yorumlar
- **Modül/etkinlik sayfaları** - bireysel etkinlik ve kaynaklardaki yorumlar
- **Her ikisi** - tüm sayfa türlerindeki yorumlar

#### Yorum Yapma Stili

Yorum yapma deneyimini seçin. Her modun ekran görüntüleri için [Yorum Yapma Stilleri](#moodle-commenting-styles) bölümüne bakın.

- **Comments** - sayfa içeriğinin altında standart, dallanmış yorum bileşeni
- **Collab Chat** - varlık göstergeleriyle satır içi metin seçimi tartışmaları
- **Her ikisi** - yorumlar ve işbirliği sohbeti birlikte etkin

#### CDN URL

FastComments CDN URL'si. Varsayılan olarak `https://cdn.fastcomments.com` değeridir. Verileriniz AB bölgesinde barındırılıyorsa bunu AB CDN URL'si ile değiştirin.