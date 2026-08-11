[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, canlı yorumlama etkinleştirilir. Bu, herhangi bir yorum eklenir, silinir, düzenlenir veya sabitlenirse, değişikliklerin aynı anda yorum dizisini izleyen tüm kullanıcılar tarafından görülmesi anlamına gelir.

Ancak, varsayılan olarak bu yeni yorumlar, “Show 2 New Comments” benzeri bir metne sahip dinamik olarak gösterilen bir düğmenin altında görünür.

Yeni yorumlar doğrudan sayfaya yanıt ise, düğme yorum dizisinin en üstünde gösterilir. Belirli bir yoruma yanıt ise, düğme o yorumun altında gösterilir.

Bu, sayfa boyutunun kullanıcıda sürekli değişmesini önlemek içindir; kaydırma çubuğunu yakalamaya çalışırken hayal kırıklığına yol açabilir.

Canlı teklif verme veya çevrimiçi etkinlikler gibi bazı kullanım senaryolarında bu istenen davranış değildir – yorum widget'ının yeni yorumların “hemen gösterildiği” bir “sohbet” kutusu gibi olmasını isteyebilirsiniz.

Bu nedenle, bu özelliği etkinleştiren bayrağın adı: **showLiveRightAway**.

Aşağıdaki gibi açabilirsiniz:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Canlı Yorumları Hemen Göster'; code-example-end]

Bu, widget özelleştirme sayfasında kod olmadan özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Canlı yorumları gizleme ayarı değiştirildi, yeni yorumlar bir düğmenin arkasında değil anında görünecek'; title='Canlı Yorumları Hemen Göster' app-screenshot-end]