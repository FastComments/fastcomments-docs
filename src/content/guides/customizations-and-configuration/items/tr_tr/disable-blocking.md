[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcıların diğer kullanıcıları engellemesine izin verir. Bir kullanıcıyı engellemek, yorumlarının gizlenmesine, kullanıcılar arasındaki bildirimlerin engellenmesine ve benzeri durumlara yol açar.

Bu işlevi devre dışı bırakmak istenebilir. Aşağıdaki gibi yapılabilir:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Engellemeyi Devre Dışı Bırak'; code-example-end]

Bu aynı zamanda kod kullanmadan, sunucu tarafı doğrulamasını da etkinleştiren, Widget Özelleştirme UI'si aracılığıyla yapılabilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Widget özelleştirme UI\'sinde engelleme seçeneği, kullanıcıların birbirini engellemesini durdurur'; title='Engellemeyi Devre Dışı Bırak' app-screenshot-end]