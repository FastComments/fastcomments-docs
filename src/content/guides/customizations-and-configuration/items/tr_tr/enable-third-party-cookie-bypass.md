[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Kimlik doğrulama için, FastComments tarayıcınızda üçüncü taraf çerezlerin etkin olmasına bağlıdır. Çerezler olmadan, kullanıcıların yorum yapmak için her zaman e-posta adreslerini bırakmaları gerekir (e-posta giriş alanı gizli değilse) ve yorumları varsayılan olarak doğrulanmamış olarak gösterilir.

Bunu aşmak için üçüncü taraf çerez atlatmasını etkinleştirebilirsiniz. 

Bu ayar etkinleştirildiğinde, kullanıcının oturum açtığını belirten bir mesaj gösteren küçük bir açılır pencere oluşur. Bu açılır pencere, kullanıcı yorum widget'ı ile etkileşime girdiğinde gösterilir; örneğin, bir yorum bıraktıklarında.

Bunu kodda **enableThirdPartyCookieBypass** bayrağını true olarak ayarlayarak yapabiliriz:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Üçüncü Taraf Çerez Atlatmasını Etkinleştirme'; code-example-end]

Bunu ayrıca Widget Özelleştirme UI'si üzerinden, `Enable Third-Party Cookie Popup` altında ayarlayabiliriz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Üçüncü Taraf Çerez Açılır Pencereyi Etkinleştir onay kutusu işaretli widget özelleştirme sayfası'; title='Üçüncü Taraf Çerez Atlatmasını Etkinleştirme' app-screenshot-end]