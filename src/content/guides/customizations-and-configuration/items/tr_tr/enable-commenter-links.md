[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcıdan yalnızca yorumunu, kullanıcı adını ve e-posta adresini ister.

Ancak bazı durumlarda kullanıcıdan kendi bloguna veya web sitesine bir bağlantı bırakmasını isteyebilirsiniz.

Kullanıcının web sitesi URL'sini bırakması için ek bir giriş alanını göstermek, **enableCommenterLinks** bayrağını true olarak ayarlayarak etkinleştirilebilir:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Belirtilen URL sağlandığında, kullanıcının hesabı güncellenecek ve geçmiş ve gelecekteki tüm yorumlarındaki kullanıcı adları bu URL'ye bağlanacaktır.

Bu, kod gerektirmeden widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---