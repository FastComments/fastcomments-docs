[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcı rozetlerini yalnızca yorum dizisindeki yorumlarında gösterir.

Ancak, bu özelliği widget özelleştirme sayfasında etkinleştirerek kullanıcı rozetlerini yorum formunun üzerindeki adlarının yanına gösterebiliriz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

Bu, kullanıcının rozetlerini adının yanında üst çubuk alanında görüntüleyerek yorum yazarken başarılarını ve durumunu daha belirgin hale getirir.

Bu özelliğin çalışması için widget özelleştirme kullanıcı arayüzünde etkinleştirilmiş olması gerektiğini unutmayın. Sunucu düzeyinde açık olsa bile, bunu seçici olarak devre dışı bırakmak için kod yapılandırmanızda isteğe bağlı olarak **showBadgesInTopBar** bayrağını false olarak ayarlayabilirsiniz:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]