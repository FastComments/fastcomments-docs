[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments yalnızca kullanıcıdan yorumunu, kullanıcı adını ve e-posta adresini ister.

Ancak, bazı durumlarda kullanıcının kendi bloguna veya web sitesine bir bağlantı bırakmasını isteyebilirsiniz.

Kullanıcının web sitesi URL'sini bırakması için ekstra bir giriş alanı göstermeyi, **enableCommenterLinks** bayrağını true olarak ayarlayarak etkinleştirebiliriz:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Yorumcu Bağlantılarını Etkinleştirme'; code-example-end]

Bu URL sağlandığında, kullanıcının hesabı güncellenecek ve geçmiş ve gelecekteki tüm yorumlarda kullanıcı adları bu URL'ye bağlanacaktır.

Bu, kod yazmadan, widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Yorum formuna bir web sitesi URL alanı eklemek için yorumcu bağlantıları onay kutusu işaretlenmiş widget özelleştirme sayfası'; title='Yorumcu Bağlantılarını Etkinleştirme' app-screenshot-end]