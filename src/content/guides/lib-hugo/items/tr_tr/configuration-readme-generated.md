Tüm FastComments widget seçenekleri `hugo.toml` içindeki `[params.fastcomments]` altında ayarlanır ve sayfa bazında front matter'da `[fastcomments]` altında geçersiz kılınabilir. Öncelik, en düşükten en yükseğe: site parametreleri, sayfa front matter'ı, shortcode parametreleri.

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

Ne `url` ne de `urlId` sağlandığında, `url` sayfanın permalink'ine varsayılan olur, böylece yorum dizileri sabit bir URL'ye bağlı kalır.

### AB veri yerleşimi

AB müşterileri widget'ı `cdn-eu.fastcomments.com`'a yönlendirmek için `region = "eu"` olarak ayarlar:

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### Anahtar yazım şekli hakkında not

Hugo, `hugo.toml` ve front matter içindeki her anahtarı küçük harfe çevirir; ancak FastComments widget'ları camelCase anahtarlara (`tenantId`, `hasDarkBackground`) ihtiyaç duyar. Bu bileşen bilinen her üst düzey seçenek için doğru yazımı otomatik olarak geri getirir, bu yüzden seçenekleri normal camelCase biçiminde yazın. Bir nesne değerinin içine gömülü anahtarlar (örneğin bir `translations` haritasının anahtarları veya `pageReactConfig` alanları) geri getirilmez. Bu türleri FastComments kontrol panelindeki özelleştirme kullanıcı arayüzü üzerinden yapılandırın.