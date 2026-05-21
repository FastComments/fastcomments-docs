The **FastComments** bloğu ana yorum arayüzüdür. Bunu blog yazısı şablonlarına, ürün şablonlarına veya tartışma dizisi ya da canlı sohbet istediğiniz herhangi bir sayfaya ekleyin.

### Bloğu ekleyin

1. Shopify tema düzenleyicisini açın (**Online Store > Themes > Customize**).
2. Yorumların görünmesini istediğiniz şablonu seçin: **Blog post**, **Product**, veya başka bir sayfa ya da bölüm şablonu.
3. Yorumların görünmesini istediğiniz bölümde **Add block** üzerine tıklayın.
4. **Apps** altında **FastComments**'ı seçin.
5. **Save**'e tıklayın.

Blok hemen görünür. Girilmesi gereken bir Tenant ID yoktur; mağazanızın tenant'ı uygulamayı yüklediğinizde otomatik olarak bağlanır.

### Ayarlar

| Setting | What it does | Default |
|---|---|---|
| Tenant ID (optional) | Bloğun hangi FastComments tenant'ı üzerinde render edileceğini geçersiz kılar. Mağazanızın otomatik olarak yapılandırılan tenant'ını kullanmak için boş bırakın. Manuel bir tenant ID'si için fastcomments.com/auth/my-account/api-secret adresine bakın. | (blank) |
| SSO | Ziyaretçiyi yorum yapmadan önce Shopify müşteri hesabı olarak otomatik oturum açtırır. Bkz. [Shopify Müşterilerinin Otomatik Girişi](/guide-installation-shopify.html#shopify-sso). | Açık |
| Commenting Style | İç içe yanıtlar ve oylar için **Threaded**, veya gerçek zamanlı sohbet akışı için **Streaming**. | Threaded |
| Custom URL ID | Otomatik algılanan sayfa tanımlayıcısını geçersiz kılar. İki URL'nin aynı yorum dizisini paylaşmasını istediğinizde bunu kullanın. | (auto-detected) |

### Sayfa tanımlayıcısı nasıl seçilir

Her yorum dizisi bir URL ID ile anahtarlanır. Blok bunu otomatik olarak seçer:

- **Blog post template:** `shopify-article-{article.id}`, bu, slug ve başlık değişiklikleri boyunca sabittir.
- **Product template:** `shopify-product-{product.id}`, bu, slug ve başlık değişiklikleri boyunca sabittir.
- **Other templates:** istek yolu (request path).

Eğer **Custom URL ID** ayarlarsanız, bunun yerine bu değer kullanılır. Birden fazla blok arasında (örneğin, ürün sayfasının yerelleştirilmiş bir varyantında) tek bir yorum dizisini paylaşmak için aynı Custom URL ID'yi kullanın.

### Threaded vs Streaming

**Threaded** varsayılandır. Ziyaretçiler birbirlerine yanıt verir, oy verir ve moderasyon araçları beklendiği gibi çalışır. Blog yazıları ve ürün incelemeleri için en uygunudur.

**Streaming** dallanmayı bırakır ve yeni yorumları gönderildikçe gerçek zamanlı olarak gösterir; bir sohbet akışı gibidir. Ürün lansmanları, canlı etkinlikler ve topluluk sayfaları için en uygunudur.

### Aynı sayfada birden fazla blok

Aynı şablona blok birden fazla kez eklenebilir. Örneğin, bir ürün sayfasının üst kısmında bir İnceleme Özeti ve altında bir FastComments bloğu olabilir. Bloklar bir URL ID paylaşır, böylece özet alttaki yorumları yansıtır.

### İpuçları

- Tema düzenleyici önizlemesinde tenant bulamazsa blok kendini sarı bir uyarı ile gizler. Bu canlı mağazanızda görünüyorsa FastComments uygulamasını yeniden yükleyin.
- Bir ürün sayfası için FastComments bloğu aynı zamanda ürün incelemeleri widget'ınız olarak da çalışır. Sayfanın en üstünde yıldız derecelendirme özeti için bunu **FastComments - Reviews Summary** ile eşleştirin.