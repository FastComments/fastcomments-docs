[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Spoiler desteğini, **enableSpoilers** bayrağını true olarak ayarlayarak etkinleştirebiliriz:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Bu işlem kod kullanmadan da yapılabilir. Widget özelleştirme sayfasında "Spoilerleri Etkinleştir" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Metin vurgulandığında ve artık görünen `SPOILER` düğmesine tıklandığında, kullanıcı fareyi üzerine getirmedikçe metin maskelenir. Karanlık mod için de aynı şeyi yapıyoruz, ancak karanlık moda daha iyi uyan farklı
renkler kullanıyoruz.

Bu aynı zamanda WYSIWYG düzenleyici ile de uyumludur.