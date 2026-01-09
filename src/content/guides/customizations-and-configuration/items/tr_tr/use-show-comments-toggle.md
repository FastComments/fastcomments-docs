[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments yorum giriş kutusunu ve yorum dizisini aynı anda gösterir. Dikey alandan tasarruf etmek için, widget ile etkileşim olana kadar diğer gerekli alanları da gizler.

Bununla birlikte, yorum widget'ı örneğin bir düğmenin arkasına gizlenebilir:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

Düğme, yorumların şu anda gösterilip gösterilmediğine bağlı olarak farklı çevrilmiş metinler kullanır. Yorumlar gizliyse, `translations.SHOW_COMMENTS_BUTTON_TEXT` kullanılır. Yorumlar gösteriliyorsa, `translations.HIDE_COMMENTS_BUTTON_TEXT` kullanılır. Çeviriler `[count]` metnini içerebilir; bu metin yerelleştirilmiş sayıyla değiştirilir.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

Bu, `hideCommentsUnderCountTextFormat` yapılandırmasının yerine geçecek şekilde tasarlanmıştır.

Sayaç, yorum dizisiyle birlikte canlı olarak güncellenir. Hiç yorum yoksa düğme gösterilmez.

Bu, bir özelleştirme kuralı oluşturarak ve "Yorumları Göstermek İçin Tıklayın" seçeneğini etkinleştirerek kod olmadan etkinleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]