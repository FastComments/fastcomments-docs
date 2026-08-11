[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments'taki biçimlendirme işlevleri, metninizin etrafına `<b></b>` gibi görünür bağlantı etiketleri ekleyerek yapılır. Araç çubuğuna tıklamak  
veya kısayolları kullanmak bunu sizin için yapar. Ancak, bazı topluluklar bağlantı etiketleri olmadan biçimlendirme kullanmak isteyebilir. Bu,  
WYSIWYG (gördüğünüz şey elde ettiğiniz şey) editörünün etkinleştirilmesi olarak adlandırılır. Bu editör, varsayılan olanla tamamen aynı görünür, ancak ekstra kod yükler  
kullanıcıların metni kalın, altı çizili vb. yapmasını, görünür bağlantı etiketleri olmadan sağlar.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'WYSIWYG Düzenlemeyi Etkinleştirme'; code-example-end]

Bu, kod olmadan da yapılabilir. Widget özelleştirme sayfasında, "Enable Advanced Formatting" seçeneğine bakın.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='WYSIWYG editörünü açmak için Gelişmiş Biçimlendirmeyi Etkinleştir onay kutusu işaretli widget özelleştirme sayfası'; title='WYSIWYG\'i Etkinleştir' app-screenshot-end]