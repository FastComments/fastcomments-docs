[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, üst düzey yorumlara yapılan yanıtlar gösterilir.

Bu, kullanıcıların çocuk yorumları görmek için üst düzey yorumlarda "Yanıtları Göster"e tıklaması gerekecek şekilde yapılandırılabilir.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

Bu, kod yazmadan, widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

Bu ayar başlangıçta yüklenen üst düzey yorumların sayısını etkilemez. Eğer bir üst düzey yorumunuz ve 29 çocuğu varsa, bu ayar açıkken şunları göreceksiniz:

- Üst düzey yorumu görürsünüz.
- Bu yorumun altında "Yanıtları Göster (29)" görürsünüz.

Bu seçenekle birlikte tüm üst düzey yorumları göstermek istiyorsanız, [başlangıç sayfasını -1 olarak ayarlayın](#starting-page).