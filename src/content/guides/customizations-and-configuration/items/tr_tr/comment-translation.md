Varsayılan olarak, FastComments widget'ın kendi arayüzünü (düğmeler, etiketler, hata mesajları) izleyicinin diline çevirir, ancak yorumlar tam olarak yazıldıkları şekilde gösterilir.

Yorum Çevirisi, yorumların kendisinin çevirisini ekler, böylece birden fazla dilde yazan bir topluluk her yorumu kendi dilinde okuyabilir.

İki mod vardır:

- **Automatic**: widget'ın dilinden farklı bir dilde yazılmış yorumlar çevrilmiş olarak gösterilir. Yorumun altında "İspanyolca'dan Çevrildi" satırı belirir ve **Show original** bağlantısı çeviri ile orijinal metin arasında geçiş yapar.
- **Manual**: her yorum bir **Translate** bağlantısı alır. Buna tıklamak, o yorumu izleyicinin diline çevirir ve ardından aynı **Show original** geçişi bulunur. Yorumlar zaten izleyicinin dilindeyse bunun yerine bunu belirtir.

Hedef dil, widget'ın dilidir: aynı özelleştirme kuralında ayarlanmışsa [Locale / Language](#locale) ayarı, aksi takdirde ziyaretçinin tarayıcı dili. Ziyaretçiden hiçbir şey istenmez, hesap, uzantı veya ekstra yazılım gerekmez ve gömülü widget'ta ve bağımsız yorum sayfalarında aynı şekilde çalışır.

Bu, <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">özelleştirme kuralı</a> başına etkinleştirilir, böylece tüm sitenize, bir domaine veya tek bir sayfaya uygulanabilir. Varsayılan olarak kapalıdır.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Widget özelleştirme sayfasında Off, Automatic ve Manual seçenekleriyle Yorum Çevirisi açılır menüsü'; title='Yorum Çevirisi' app-screenshot-end]

Çeviri, sunucu tarafı bir ayardır. Widget'ın JavaScript yapılandırmasında `commentTranslation` geçmek bunu etkinleştirmez; kural bunu yapar.

Her yorum, hedef dil başına en fazla bir kez çevrilir ve sonuç yorumla birlikte depolanır, böylece binlerce İspanyolca konuşanı tarafından okunan bir yorum bir kez çevrilir. Yazar veya bir moderatör yorumu düzenlerse, depolanan çeviri atılır ve yorum bir sonraki görüntülemede tekrar çevrilir. Çevrilen metin, orijinal yorumla aynı temizlik ve bağlantı kurallarına tabi tutulur. Çok uzun olan veya dili tespit edilemeyen yorumlar çevrilmemiş olarak gösterilir.

Kullanım, [AI Agents](/guide-ai-agents.html#plans-and-eligibility) ile aynı oran üzerinden LLM tokenları olarak faturalandırılır ve fatura detaylarınızda aynı yerde görünür.