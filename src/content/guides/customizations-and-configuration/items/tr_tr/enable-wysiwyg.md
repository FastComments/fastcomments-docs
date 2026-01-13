[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Varsayılan olarak FastComments'taki biçimlendirme işlevleri, metninizin etrafına `<b></b>` gibi görünür etiketler ekleyerek yapılır. Araç çubuğuna tıklamak veya kısayolları kullanmak bunu sizin için yapar. Ancak bazı topluluklar, biçimlendirmeyi bağlantı etiketleri olmadan kullanmayı tercih edebilir. Buna WYSIWYG (gördüğünüz şey, elde ettiğiniz şey) düzenleyicisini etkinleştirmek denir. Bu düzenleyici görünüş olarak varsayılan olanla tamamen aynıdır; farkı, kullanıcıların metinlerini görünür etiketler olmadan kalın, altı çizili vb. yapmalarını sağlayan ek kodu yüklemesidir.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında "Gelişmiş Biçimlendirmeyi Etkinleştir" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---