[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, yerelleştirilmiş göreli tarihler kullanılır. Örneğin, yeni bırakılmış bir yorumun yanında "11 dakika önce" görebilirsiniz.

Mutlak tarihleri kullanmak gerekli veya istenebilir; bu durumda bu parametreyi true olarak ayarlarsınız. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Mutlak Tarihleri Kullan'; code-example-end]

Bu, kod olmadan, widget özelleştirme sayfasında Gelişmiş Seçenekler altında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='Mutlak tarih geçişi açık olan widget özelleştirme sayfasındaki Gelişmiş Seçenekler'; title='Mutlak Tarihleri Kullan' app-screenshot-end]