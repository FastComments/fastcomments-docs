[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Varsayılan olarak FastComments, her yorumu kimin görüntülediğini izlemez veya bununla ilgili herhangi bir istatistik sağlamaz.

Ancak bu özelliği etkinleştirebiliriz; sistem, her kullanıcı bir yoruma kaydırdıkça izlemeye başlayacaktır.

Bu gerçekleştiğinde, her yorumda gösterilen bir göz simgesinin yanındaki sayı artırılacaktır. Sayı canlı olarak güncellenir ve kullanıcının yerel ayarına göre kısaltılır.

Bunu **enableViewCounts** bayrağını true olarak ayarlayarak etkinleştirebiliriz:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Bu, widget özelleştirme sayfasında kod yazmadan özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Yorumu görüntüleyen kullanıcı kimliğini* izliyoruz; böylece yorumu tekrar görüntülerseniz sayı artmaz. Yorumu iki yıl sonra tekrar görüntülerseniz, sayı yeniden artar.

- *Not: veya anonim oturum kimliği, veya kullanıcının IP'sinin hashlenmiş değeri.

---