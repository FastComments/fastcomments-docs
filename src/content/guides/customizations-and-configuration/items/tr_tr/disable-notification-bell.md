[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments yorum alanının sağ üst köşesinde bir bildirim zili gösterir.

Bu zil kırmızıya döner ve kullanıcının sahip olduğu bildirim sayısını gösterir. Örnek bildirimler şunlardır:

- Kullanıcı size yanıt verdi.
- Kullanıcı, yorum yaptığınız bir dizide yanıt verdi.
- Kullanıcı yorumunuzu beğendi.
- Kullanıcı, abone olduğunuz bir sayfaya yanıt verdi.

Bildirim zili aynı zamanda bir bütün sayfaya abone olma mekanizması da sağlar.

Bununla birlikte, bildirim zilini tamamen devre dışı bırakabiliriz:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Bildirim Zilini Devre Dışı Bırak'; code-example-end]

Bu aynı zamanda kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Bildirim Zilini Devre Dışı Bırak" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Bildirim Zilini Devre Dışı Bırak kutucuğu işaretli widget özelleştirme sayfası'; title='Bildirim Zilini Devre Dışı Bırak' app-screenshot-end]