[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments'te canlı yorumlama etkinleştirilmiştir.

Bu, yorum dizisini görüntüleyen her kullanıcının aynı içeriği görmesi anlamına gelir.

Örneğin, bir yorum eklendiğinde, o yorum görünmelidir. Bir yorum düzenlenir veya kaldırılırsa,
o yorumlar yorum dizisini görüntüleyen tüm kullanıcılar için düzenlenir veya kaldırılır. Oylar ve tüm moderasyon işlemleri için de aynı durum geçerlidir.

Ancak, bunu devre dışı bırakabiliriz:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında "Canlı Yorumlamayı Devre Dışı Bırak" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]