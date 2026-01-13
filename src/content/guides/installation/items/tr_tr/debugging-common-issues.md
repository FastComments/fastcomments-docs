İşte sık karşılaştığımız bazı belirtiler ve yaygın çözümler.

### "This is a demo" Mesajı

Bu, demo kiracımızı kullanan ana sayfamızdan widget kodunu kopyaladığınızda gösterilir. Kiracınızı kullanmak için widget kodunu [buradan](https://fastcomments.com/auth/my-account/get-acct-code) kopyalayın.

### "FastComments cannot load on this domain" Hatası

FastComments, hesabınızla ilişkili istekleri doğrulamak için hangi alan adlarının size ait olduğunu bilmelidir. Bu hatayı nasıl çözeceğinizi öğrenmek için [dokümantasyonumuza göz atın](/guide-multiple-sites.html#add-domains-to-account) (sadece tam alt alan adı + alan adını hesabınıza ekleyin).

Bunun yalnızca deneme süresi sona erdikten sonra olması gerektiğini unutmayın. Deneme süresi boyunca, yeni alan adlarından gelen tüm istekler otomatik olarak hesabınıza eklenir.

### Özel Kurulumlar İçin Taşınan Yorumlar Görünmüyor

Bu genellikle içe aktarılan yorumlar bir `Page ID`'ye bağlıyken siz URL geçirdiğinizde (veya hiç değer geçirmediğinizde, bu durumda varsayılan olarak sayfa URL'si kullanılır) olur.

Bunu [yorumlarınızı dışa aktararak](https://fastcomments.com/auth/my-account/manage-data/export) ve `URL ID` sütununu (şu anda Sütun `B`) görüntüleyerek hata ayıklayabilirsiniz.

`URL ID` sütununda gördüğünüz değerlerin, widget yapılandırmasına `urlId` parametresi olarak geçirdiğiniz değerlerle aynı olduğundan emin olun.

Daha fazla açıklama için [Yorumların Sayfalara ve Makalelere Nasıl Bağlandığı dokümantasyonumuzu](/guide-customizations-and-configuration.html#url-id) okumayı deneyin.

Her şey başarısız olursa, [bizimle iletişime geçin](https://fastcomments.com/auth/my-account/help).

### Yorum Widget'ı Görünmüyor

Yorum widget'ı görünmüyorsa, Chrome geliştirici konsolunda hataları kontrol edin.

Çoğu yanlış yapılandırmada, yorum widget'ı yüklenebildiyse en azından sayfada bir hata gösterir. Hiçbir şey görmemek genellikle bir betik hatasının işaretidir.

### İstenen Yapılandırma Beklendiği Gibi Çalışmıyor

Yorum widget'ına hangi yapılandırmanın geçirildiğini görmek için [Chrome uzantımızı](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) deneyin. Her şey başarısız olursa, Chrome uzantısının gösterdiğinin ekran görüntüsünü alın ve [bizimle iletişime geçin](https://fastcomments.com/auth/my-account/help).

### Farklı Hash Bang ile Aynı URL'de Eksik Yorumlar

Varsayılan olarak, FastComments yorumların depolandığı "kova" olarak sayfa URL'sini kullanır. URL'leriniz `#hashbangs` içeriyorsa ve bu `#hashbangs` yorum dizisini tanımlayan tanımlayıcının bir parçası olmamalıysa, hash bang değerini basitçe yok sayabiliriz, örneğin:

[inline-code-attrs-start title = 'Hash Bangleri Yoksay Örneği'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Bu değişikliği yaptıktan sonra mevcut yorumlar için bir taşıma yapılması gerekeceğini unutmayın. [Bunun için bizimle iletişime geçin.](https://fastcomments.com/auth/my-account/help)

### URL Sorgu Parametreleri Widget'ı Etkiliyor

Varsayılan olarak, FastComments yorumların depolandığı "kova" olarak sayfa URL'sini kullanır. URL'leriniz yorum dizisini tanımlayan tanımlayıcının bir parçası olmaması gereken sorgu parametreleri içeriyorsa, bunları basitçe yok sayabiliriz, örneğin:

[inline-code-attrs-start title = 'Sorgu Parametrelerini Yoksay'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Bu değişikliği yaptıktan sonra mevcut yorumlar için bir taşıma yapılması gerekeceğini unutmayın. [Bunun için bizimle iletişime geçin.](https://fastcomments.com/auth/my-account/help)

### E-posta Almıyorsunuz

FastComments'ta, e-posta teslimatımızın olabildiğince güvenilir olmasını sağlamak için çok çalışıyoruz. Ancak, bazı e-posta sağlayıcılarının güvenilir bir şekilde teslim edilmesi zordur. Spam klasörünüzde fastcomments.com'dan gelen mesajları kontrol edin.

[Bizimle iletişime geçerseniz](https://fastcomments.com/auth/my-account/help), genellikle e-postalarımızı neden göremediğiniz hakkında daha fazla bilgi sağlayabiliriz.
