[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Varsayılan olarak, FastComments yorum widget'ı `pg` bir `gif rating` ayarlar.

Mevcut seçenekler `g`, `pg`, `pg-13` ve `r`'dir.

Bu, kod içinde veya UI üzerinden ayarlanabilir. Koddaki örnek aşağıdaki gibidir:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Gif Değerlendirmesini Ayarla'; code-example-end]

UI'da, `Disable Image Uploads?` işaretli olmadığı sürece, bunu `Gif Picker Rating` altında bulabilirsiniz.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Widget özelleştirme sayfasında g, pg, pg-13 ve r seçeneklerini sunan Gif Picker Rating açılır menüsü'; title='Gif Değerlendirmesini Ayarlama' app-screenshot-end]