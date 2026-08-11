[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcı profillerinde bir "Direct Messages" sekmesi gösterir ve ziyaretçilerin bir kullanıcıya doğrudan mesaj göndermesine izin verir.

Ancak, bu sekmeyi devre dışı bırakabiliriz:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Bu aynı zamanda kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Disable Direct Messages" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Widget özelleştirme sayfası, profil mesajları sekmesini gizlemek için Disable Direct Messages kutusunun işaretli olduğu durumda'; title='Disable Profile Direct Messages' app-screenshot-end]