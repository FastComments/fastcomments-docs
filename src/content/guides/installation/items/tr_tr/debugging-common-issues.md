İşte sıkça karşılaştığımız bazı belirtiler ve yaygın çözümler. 

### "This is a demo" Mesajı

Bu, widget kodunu ana sayfamızdan kopyaladığınızda gösterilir; ana sayfamız demo kiracımızı kullanır. Kendi kiracınızı kullanmak için widget kodunu buradan kopyalayın: [here](https://fastcomments.com/auth/my-account/get-acct-code).

### "FastComments cannot load on this domain" Hatası

FastComments, hesabınızla ilişkili istekleri kimlik doğrulamak için hangi alan adlarının size ait olduğunu bilmelidir. Bu hatayı nasıl çözeceğinizi görmek için belgelerimize bakın: [Check out our documentation](/guide-multiple-sites.html#add-domains-to-account) (hesabınıza tam alt alan + alan adını ekleyin).

Bu durumun yalnızca deneme süresi bittikten sonra meydana gelmesi gerektiğini unutmayın. Deneme süresi boyunca yeni alan adlarından gelen istekler hesabınıza otomatik olarak eklenecektir.

### Migrated Comments Not Showing for Custom Installations

Genelde bu, içe aktarılan yorumların bir `Page ID` ile ilişkilendirilmiş olması ve sizden bir URL (veya herhangi bir değer yoksa varsayılan olarak sayfa URL'si) geçiriliyor olmasından kaynaklanır.

Bunu hata ayıklamak için [yorumlarınızı dışa aktarın](https://fastcomments.com/auth/my-account/manage-data/export) ve `URL ID` sütununu (şu anda Sütun `B`) görüntüleyin.

`URL ID` sütununda gördüğünüz değerlerin, widget yapılandırmasına `urlId` parametresi olarak geçirdiğiniz değerlerle aynı olduğundan emin olun.

Daha fazla açıklama için, lütfen bizim [How Comments are Tied to Pages and Articles documentation](/guide-customizations-and-configuration.html#url-id) sayfamızı okuyun.

Eğer her şey başarısız olursa, [reach out to us](https://fastcomments.com/auth/my-account/help).

### Comment Widget Not Showing

Eğer yorum widget'ı görünmüyorsa, hatalar için Chrome geliştirici konsolunu kontrol edin.

Çoğu yapılandırma hatasında, yorum widget'ı en azından sayfada yükleyebiliyorsa bir hata gösterir. Hiçbir şey görmemek genellikle bir betik hatasının göstergesidir.

### Desired Configuration Not Working as Expected

Yorum widget'ına hangi yapılandırmanın geçirildiğini görmek için [Chrome uzantımızı](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US) deneyin. Eğer yine de sorun çözülmezse, Chrome uzantısının gösterdiklerinin bir ekran görüntüsünü alın ve [reach out to us](https://fastcomments.com/auth/my-account/help).

### Comments Missing on Same URL With Different Hash Bang

Varsayılan olarak, FastComments yorumların saklandığı "kova" için sayfa URL'sini kullanacaktır. Eğer URL'leriniz `#hashbang` içeriyorsa ve bu `#hashbang`ler bir yorum dizisini tanımlayan tanımlayıcının parçası olmamalıysa, hash bang değerini basitçe yok sayabiliriz, örneğin:

[inline-code-attrs-start title = 'Hash Bangleri Yoksayma Örneği'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bu değişikliği yaptıktan sonra mevcut yorumlar için bir taşıma işlemi yapılması gerektiğini unutmayın. [Bunun için bize ulaşın.](https://fastcomments.com/auth/my-account/help)

### URL Query Parameters Affecting Widget

Varsayılan olarak, FastComments yorumların saklandığı "kova" için sayfa URL'sini kullanacaktır. Eğer URL'leriniz kimlik doğrulayıcı bir yorum dizisini tanımlayan tanımlayıcıya dahil edilmemesi gereken sorgu parametreleri içeriyorsa, bunları basitçe yok sayabiliriz, örneğin:

[inline-code-attrs-start title = 'Sorgu Parametrelerini Yoksayma'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

Bu değişikliği yaptıktan sonra mevcut yorumlar için bir taşıma işlemi yapılması gerektiğini unutmayın. [Bunun için bize ulaşın.](https://fastcomments.com/auth/my-account/help)

### Not Receiving Emails

FastComments olarak e-postaların teslimatını mümkün olduğunca güvenilir hale getirmek için çok çalışıyoruz. Ancak bazı e-posta sağlayıcılarına güvenilir şekilde teslim etmek özellikle zordur. fastcomments.com'dan gelen mesajlar için spam klasörünüzü kontrol edin.

Eğer [bize ulaşırsanız](https://fastcomments.com/auth/my-account/help), genellikle neden bizim gönderdiğimiz e-postaları görmediğiniz konusunda daha fazla bilgi sağlayabiliriz.