[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments her yorumun kim tarafından görüntülendiğini izlemez veya bununla ilgili istatistikler sağlamaz.

Ancak bu özelliği etkinleştirebiliriz ve sistem, her kullanıcı bir yoruma kaydırdıkça izlemeye başlayacaktır.

Bu gerçekleştiğinde, her yorumda gösterilen göz simgesinin yanındaki sayı artar. Sayı, canlı olarak güncellenir ve kullanıcının yerel ayarına göre kısaltılır.

Bu özelliği **enableViewCounts** bayrağını true olarak ayarlayarak etkinleştirebiliriz:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Yorum Görüntüleme Sayılarını Etkinleştirme'; code-example-end]

Bu, kod yazmadan widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Görünüm sayıları onay kutusu işaretli widget özelleştirme sayfası, böylece her yorum bir göz simgesi ve sayı gösterir'; title='Yorum Görüntüleme Sayılarını Etkinleştirme' app-screenshot-end]

Yorumu görüntüleyen kullanıcı kimliğini* izliyoruz, böylece yorumu tekrar görüntülerseniz sayı artmaz. Yorumu iki yıl sonra tekrar görüntülerseniz, sayı daha fazla artar.

- *Not: anonim oturum kimliği veya kullanıcının IP'si hashlenmiş bir değer olabilir.