[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcı rozetlerini yalnızca yorum dizisindeki yorumlarında gösterir.

Ancak, bu özelliği widget özelleştirme sayfasında etkinleştirerek kullanıcı rozetlerini yorum formunun üzerindeki isimlerinin yanına gösterebiliriz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='Widget özelleştirme sayfasındaki üst çubukta rozetleri göster onay kutusu, rozetleri yorum formunun üzerindeki ismin yanına yerleştirir'; title='Üst Çubukta Rozetleri Göster Seçeneği' app-screenshot-end]

Bu, kullanıcının rozetlerini adının yanında üst çubuk alanında gösterir ve yorum yazarken başarılarını ve durumunu daha belirgin hâle getirir.

Bu özelliğin çalışması için widget özelleştirme arayüzünde etkinleştirilmiş olması gerektiğini unutmayın. Sunucu seviyesinde açık olsa bile, kod yapılandırmanızda **showBadgesInTopBar** bayrağını false olarak ayarlayarak isteğe bağlı olarak devre dışı bırakabilirsiniz:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]