FastComments SSR'yi kullanmak için istemci, `https://fastcomments.com/ssr/comments` uç noktasından HTML alabilir.

Bu birkaç farklı şekilde yapılabilir.

### WordPress ile

WordPress eklentisinde `3.10.2` sürümünden itibaren, JS'siz kullanıcılar için SSR varsayılan olarak yedek mekanizma (fallback) olarak etkindir.

### Bir Web Sayfasında

Mevcut bir uygulama ile SSR, kullanılan dilin PHP olduğu varsayımıyla [aşağıdaki örnekle](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr) eklenebilir:

[inline-code-attrs-start title = 'PHP Tabanlı SSR Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<iframe
    src="<?php echo $fastcomments_url; ?>"
    horizontalscrolling="no"
    allowtransparency="true"
    frameborder="0"
    title="FastComments"
    width="100%"
    height="1500px"
    style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
></iframe>
[inline-code-end]

Ayrıca SSR arayüzünü yalnızca kullanıcının JS'i devre dışı olduğunda gösterebiliriz:

[inline-code-attrs-start title = 'PHP Tabanlı SSR Yedek Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
$tenantId = rawurlencode("my-tenant-id");
$urlId = rawurlencode("my-page-url-article-id");
$url = rawurlencode("my-page-url");

$fastcomments_url = "https://fastcomments.com/ssr/comments?tenantId=$tenantId&urlId=$urlId&url=$url";
?>
<noscript>
    <iframe
        src="<?php echo $fastcomments_url; ?>"
        horizontalscrolling="no"
        allowtransparency="true"
        frameborder="0"
        title="FastComments"
        width="100%"
        height="1500px"
        style= "width: 1px !important; min-width: 100% !important; border: none !important; overflow: hidden !important;"
    ></iframe>
</noscript>
[inline-code-end]

SSO kullanan bir örnek için [buraya bakın](https://github.com/FastComments/fastcomments-code-examples/tree/master/sso/php/ssr).

### Ön İşlenmiş (Pre-Rendered) İçerikle

Blogumuz derleme zamanında oluşturulur ve [Handlebars ile SSR'nin iyi bir örneğini](https://github.com/FastComments/fastcomments-blog/blob/master/src/templates/post.html#L51) sunar.

### Temel Parametreler

Geçmeniz gereken temel parametreler şunlardır:
- `tenantId` - Sizi müşteri olarak tanımlar.
- `urlId` - Yorumların yükleneceği sayfayı veya makaleyi tanımlar ve nereye kaydedileceklerini belirler.
- `url` - Bildirimler ve ilgili özellikler tarafından yorum dizisine geri bağlantı vermek için kullanılır.

### Özel Stil

Yorum bileşeninin SSR sürümü, JavaScript sürümüyle aynı yapıyı ve işleme motorunu kullanır.

Bu nedenle, JavaScript yorumlama bileşeni için çalışan tüm özel stiller SSR için de çalışır.

### Notlar

SSR ile birlikte, işlenen konteynerin yüksekliğini kontrol edecek bir JavaScript bulunmaz. Tarayıcılarda, uzun tartışmalar için dikey bir kaydırma çubuğu görünebilir.

Bu nedenle, bunu istediğiniz gibi ayarlamanız gerekir.