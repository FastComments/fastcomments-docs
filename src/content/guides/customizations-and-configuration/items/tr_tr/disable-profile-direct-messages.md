[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcı profillerinde "Doğrudan Mesajlar" sekmesini gösterir; bu, ziyaretçilerin bir kullanıcıya doğrudan mesaj göndermesine olanak tanır.

Ancak, bu sekmeyi devre dışı bırakabiliriz:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Bu, kod kullanmadan da yapılabilir. Widget özelleştirme sayfasında, "Doğrudan Mesajları Devre Dışı Bırak" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]