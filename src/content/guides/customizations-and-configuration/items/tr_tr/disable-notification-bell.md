---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments yorum alanının sağ üst köşesinde bir bildirim zili gösterir.

Bu zil kırmızı olur ve kullanıcının sahip olduğu bildirim sayısını gösterir. Bazı örnek bildirimler şunlardır:

- Kullanıcı size yanıt verdi.
- Kullanıcı, yorum yaptığınız bir başlıkta yanıt verdi.
- Kullanıcı yorumunuza oy verdi.
- Kullanıcı, abone olduğunuz bir sayfaya yanıt verdi.

Bildirim zili ayrıca bir sayfaya abone olma mekanizması da sağlar.

Ancak, bildirim zilini tamamen devre dışı bırakabiliriz:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Bu kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Bildirim Zilini Devre Dışı Bırak" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---