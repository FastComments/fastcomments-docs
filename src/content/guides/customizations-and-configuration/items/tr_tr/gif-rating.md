[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

Varsayılan olarak, FastComments yorum widget'ı `gif rating`'i `pg` olarak ayarlar.

Mevcut seçenekler `g`, `pg`, `pg-13` ve `r`'dir.

Bu kodda veya kullanıcı arayüzü üzerinden ayarlanabilir. Kodda şu şekilde yapabiliriz:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

Kullanıcı arayüzünde, `Gif Picker Rating` altında bulacaksınız; ancak `Disable Image Uploads?` işaretli olmadıkça.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]