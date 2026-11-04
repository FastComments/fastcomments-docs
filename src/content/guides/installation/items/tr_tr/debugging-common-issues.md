Sık karşılaştığımız bazı belirtiler ve yaygın çözümleri aşağıda bulabilirsiniz.

### "This is a demo" Mesajı

Bu mesaj, widget kodunu ana sayfamızdan kopyaladığınızda görünür; ana sayfadaki kod demo kiracımızı (tenant) kullanır. Kendi kiracınızı kullanmak için widget kodunu [buradan](https://fastcomments.com/auth/my-account/get-acct-code) kopyalayın.

### "FastComments cannot load on this domain" Hatası

FastComments, hesabınızla ilişkili isteklerin kimliğini doğrulamak için hangi alan adlarının size ait olduğunu bilmelidir. Bu hatayı nasıl çözeceğinizi görmek için [dokümantasyonumuza göz atın](/guide-multiple-sites.html#add-domains-to-account) (basitçe tam alt alan adı + alan adını hesabınıza ekleyin).

Bunun yalnızca deneme süresi bittikten sonra gerçekleşmesi gerektiğini unutmayın. Deneme süresi boyunca, yeni alan adlarından gelen istekler otomatik olarak hesabınıza eklenir.

### Özel Kurulumlarda Taşınan Yorumların Görünmemesi

Genellikle bu, içe aktarılan yorumların bir `Page ID` değerine bağlı olması ve sizin bir URL iletmeniz (veya hiçbir değer iletmemeniz, bu durumda sayfa URL'si varsayılan olarak kullanılır) durumunda gerçekleşir.

Bunu, [yorumlarınızı dışa aktararak](https://fastcomments.com/auth/my-account/manage-data/export) ve `URL ID` sütununu (şu anda `B` sütunu) görüntüleyerek hata ayıklayabilirsiniz.

`URL ID` sütununda gördüğünüz değerlerin, widget yapılandırmasına `urlId` parametresi olarak ilettiğiniz değerlerle aynı olduğundan emin olun.

Daha fazla açıklama için [Yorumların Sayfalara ve Makalelere Nasıl Bağlandığı dokümantasyonumuzu](/guide-customizations-and-configuration.html#url-id) okumayı deneyin.

Her şey başarısız olursa, [bize ulaşın](https://fastcomments.com/auth/my-account/help).

### Yorum Widget'ının Görünmemesi

Yorum widget'ı görünmüyorsa, hatalar için Chrome geliştirici konsolunu kontrol edin.

Çoğu yanlış yapılandırma durumunda, yorum widget'ı yükleyebilirse sayfada en azından bir hata gösterir. Hiçbir şey görmemek genellikle bir betik (scripting) hatasına işarettir.

### İstenen Yapılandırmanın Beklendiği Gibi Çalışmaması

Yorum widget'ına hangi yapılandırmanın iletildiğini görmek için [Chrome uzantımızı](https://chromewebstore.google.com/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj) deneyin. Her şey başarısız olursa, Chrome uzantısının gösterdiklerinin ekran görüntüsünü alın ve [bize ulaşın](https://fastcomments.com/auth/my-account/help).

### Aynı URL'de Farklı Hash Bang ile Yorumların Eksik Olması

Varsayılan olarak, FastComments yorumların saklandığı "bucket" için sayfa URL'sini kullanır. URL'leriniz `#hashbang` değerleri içeriyorsa ve bu `#hashbang` değerleri bir yorum dizisini tanımlayan tanımlayıcının parçası olmamalıysa, hash bang değerini basitçe yok sayabiliriz, örneğin:

[inline-code-attrs-start title = 'Hash Bang Yok Sayma Örneği'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Bu değişikliği yaptıktan sonra, mevcut yorumlar için bir taşıma (migration) işleminin gerçekleştirilmesi gerekeceğini unutmayın. [Bunun için bize ulaşın.](https://fastcomments.com/auth/my-account/help)

### URL Sorgu Parametrelerinin Widget'ı Etkilemesi

Varsayılan olarak, FastComments yorumların saklandığı "bucket" için sayfa URL'sini kullanır. URL'leriniz, bir yorum dizisini tanımlayan tanımlayıcının parçası olmaması gereken sorgu parametreleri içeriyorsa, bunları basitçe yok sayabiliriz, örneğin:

[inline-code-attrs-start title = 'Sorgu Parametrelerini Yok Sayma'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Bu değişikliği yaptıktan sonra, mevcut yorumlar için bir taşıma (migration) işleminin gerçekleştirilmesi gerekeceğini unutmayın. [Bunun için bize ulaşın.](https://fastcomments.com/auth/my-account/help)

### E-posta Alınamaması

FastComments olarak, e-postalarımızın teslimatını mümkün olduğunca güvenilir hale getirmek için çok çalışıyoruz. Ancak, bazı e-posta sağlayıcıları güvenilir teslimat için zorlu olarak bilinir. fastcomments.com'dan gelen mesajlar için spam klasörünüzü kontrol edin.

[Bize ulaşırsanız](https://fastcomments.com/auth/my-account/help), genellikle e-postalarımızı neden göremeyebileceğiniz konusunda daha fazla bilgi sağlayabiliriz.