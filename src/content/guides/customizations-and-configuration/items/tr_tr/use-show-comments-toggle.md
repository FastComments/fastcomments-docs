[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments yorum giriş kutusunu ve yorum dizisini aynı anda render eder. Dikey alan tasarrufu sağlamak için,
widget ile etkileşime girilene kadar diğer gerekli alanları da gizler.

Ancak, yorum widget'ı bir düğmenin arkasında gizlenebilir, örneğin:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Yorum widget\'ı, okuyucu tıklayana kadar yorum sayısını gösteren bir düğmenin arkasında gizlenmiş'; title='Yorumları Göstermek İçin Tıklayın' app-screenshot-end]

Düğme, yorumların şu anda gösterilip gösterilmediğine bağlı olarak farklı çevrilmiş metinler kullanır. Yorumlar gizli ise `translations.SHOW_COMMENTS_BUTTON_TEXT` kullanılır. Yorumlar gösteriliyorsa `translations.HIDE_COMMENTS_BUTTON_TEXT` kullanılır. Çeviriler, yerelleştirilmiş sayıyla değiştirilecek `[count]` metnini içerebilir.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Yorumları Göster veya Gizle'; code-example-end]

Bu, `hideCommentsUnderCountTextFormat` yapılandırmasını değiştirmek için tasarlanmıştır.

Sayı, yorum dizisiyle birlikte canlı olarak güncellenir. Yorum yoksa düğme gösterilmez.

Bu, bir özelleştirme kuralı oluşturarak ve "Yorumları Göster" seçeneğini etkinleştirerek kod olmadan etkinleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Widget özelleştirme sayfasında bir özelleştirme kuralında yorumları göster onay kutusunun işaretlenmiş hali'; title='Yorumları Göster\'i Etkinleştir' app-screenshot-end]