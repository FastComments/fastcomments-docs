[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments her yorumun kim tarafından görüntülendiğini izlemiyor ve bununla ilgili istatistikler sağlamıyor.

Ancak bu özelliği etkinleştirebiliriz ve sistem, her kullanıcı bir yoruma kaydırdıkça izlemeye başlayacaktır.

Bu gerçekleştiğinde, her yorumda gösterilen göz simgesinin yanındaki sayı artar. Sayı, canlı olarak güncellenir ve kullanıcının yerel ayarına göre kısaltılır.

Bunu, **enableViewCounts** bayrağını true olarak ayarlayarak etkinleştirebiliriz:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Yorum Görüntüleme Sayılarını Etkinleştirme'; code-example-end]

Bu, kod kullanmadan, widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Görünüm sayısı onay kutusu işaretli widget özelleştirme sayfası, böylece her yorum bir göz simgesi ve sayı gösterir'; title='Yorum Görüntüleme Sayılarını Etkinleştirme' app-screenshot-end]

Yorumu görüntüleyen kullanıcı kimliğini* bir hafta boyunca izliyoruz, böylece aynı hafta içinde yorumu tekrar görüntülerseniz sayı artmaz. Hafta geçtikten sonra yorumu tekrar görüntülerseniz, sayı tekrar artar.

- *Not: anonim oturum kimliği veya kullanıcının IP'si hashlenmiş bir değer olabilir.