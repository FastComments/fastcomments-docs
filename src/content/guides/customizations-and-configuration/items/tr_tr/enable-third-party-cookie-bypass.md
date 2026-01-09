[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Kimlik doğrulama için FastComments, tarayıcınızda üçüncü taraf çerezlerinin etkin olmasına bağlıdır. Bunlar olmadan, kullanıcılar her zaman e-posta bırakmak zorunda
kalmaktadır yorum yapmak için (e-posta giriş alanı gizlenmedikçe) ve yorumları her zaman doğrulanmamış olarak görünecektir (varsayılan olarak).

Bunun üstesinden gelmek için üçüncü taraf çerez atlamasını etkinleştirebilirsiniz. 

Bu ayar etkinleştirildiğinde, kullanıcının giriş yapıldığına dair bir mesaj gösteren küçük bir açılır pencere oluşturur. Bu açılır pencere
kullanıcı yorum bileşeniyle etkileşime girdiği her seferde gösterilir; örneğin, bir yorum bıraktığında.

Bunu kodda **enableThirdPartyCookieBypass** bayrağını true olarak ayarlayarak yapabiliriz:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Bunu ayrıca Widget Özelleştirme UI üzerinden, `Enable Third-Party Cookie Popup` altında da ayarlayabilirsiniz:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---