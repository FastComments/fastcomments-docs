[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments kullanıcıların diğer kullanıcıları engellemesine izin verir. Bir kullanıcıyı engellemek, yorumlarının
gizlenmesine, kullanıcılar arasında bildirimlerin engellenmesine ve benzeri durumlara neden olur.

Bu işlevi devre dışı bırakmak isteyebilirsiniz. Şöyle yapılabilir:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Bu, ayrıca uygun sunucu tarafı doğrulamasını etkinleştirerek, Widget Özelleştirme UI'si aracılığıyla kod yazmadan da yapılabilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---