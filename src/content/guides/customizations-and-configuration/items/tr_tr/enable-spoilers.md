[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

**enableSpoilers** bayrağını true olarak ayarlayarak spoiler desteğini etkinleştirebiliriz:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Spoiler\'ları Etkinleştirme'; code-example-end]

Bu, kod yazmadan da yapılabilir. Widget özelleştirme sayfasında, "Enable Spoilers" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Widget özelleştirme sayfası, Spoiler\'ları Etkinleştir onay kutusu işaretli ve editöre SPOILER düğmesi eklenmiş'; title='Spoiler\'ları Etkinleştir' app-screenshot-end]

Metin vurgulandığında ve artık görünen `SPOILER` düğmesine tıklandığında, metin kullanıcı üzerine gelene kadar maskelenir. Karanlık mod için aynı şeyi yaparız, ancak karanlık moda daha iyi uyan farklı renklerle.

Bu aynı zamanda WYSIWYG editörüyle de uyumludur.