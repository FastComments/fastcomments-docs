[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments canlı yorumlamayı etkinleştirir.

Bu, yorum dizisinin her izleyicisinin aynı içeriği görmesi gerektiği anlamına gelir.

Örneğin, bir yorum eklendiğinde, o yorum gösterilmelidir. Bir yorum düzenlenir veya kaldırılırsa,
bu yorumlar dizinin tüm izleyicileri için de düzenlenir veya kaldırılır. Oylar ve tüm moderasyon eylemleri de aynı şekilde.

Ancak, bunu devre dışı bırakabiliriz:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Disable Live Commenting" bölümüne bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Widget özelleştirme sayfasının Canlı Yorumlamayı Devre Dışı Bırakma bölümü, gerçek zamanlı dizi güncellemelerini kapatır'; title='Canlı Yorumlamayı Devre Dışı Bırak' app-screenshot-end]