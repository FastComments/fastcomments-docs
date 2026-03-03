[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcı profillerinde "Profile Comments" sekmesini gösterir, ziyaretçilerin birinin profiline yorum bırakmasına olanak tanır.

Ancak bu sekmeyi devre dışı bırakabiliriz:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

Bu işlem kod kullanmadan da yapılabilir. Widget özelleştirme sayfasında "Disable Profile Comments" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; title='Disable Profile Comments' app-screenshot-end]