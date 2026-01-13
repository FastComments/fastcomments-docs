[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Varsayılan olarak canlı yorumlama etkinleştirilmiştir. Bu, herhangi bir yorum eklendiğinde, silindiğinde, düzenlendiğinde veya sabitlendiğinde değişikliklerin görünmesi gerektiği anlamına gelir
yorum dizisini aynı anda görüntüleyen tüm kullanıcılara.

Ancak, varsayılan olarak bu yeni yorumlar, "Show 2 New Comments" gibi bir metne sahip dinamik olarak gösterilen bir düğmenin altında görünecektir.

Yeni yorumlar doğrudan sayfaya yapılan yanıtlar ise, düğme yorum dizisinin en üstünde gösterilir. Eğer belirli bir yoruma yanıt ise, 
düğme o yorumun altında gösterilir.

Bu, sayfa boyutunun kullanıcının ekranında sürekli değişmesini önlemek içindir; bu, kaydırma çubuğunu yakalamaya çalışırken hayal kırıklığına neden olabilir.

Bazı kullanım durumları için, örneğin canlı teklif verme veya çevrimiçi etkinlikler gibi, bu istenen davranış değildir - yorum bileşeninin
yeni yorumların "show right away" olduğu bir "chat" kutusuna daha çok benzemesini isteyebilirsiniz.

Bu özelliği etkinleştiren bayrağın adı şudur: **showLiveRightAway**.

Şu şekilde açabiliriz:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Bu, kod olmadan, bileşen özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]