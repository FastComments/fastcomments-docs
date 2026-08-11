[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Yorum yapma, readonly bayrağı true olarak ayarlandığında yeni yorumların veya oyların bırakılmasını engelleyecek şekilde kilitlenebilir.

Yorumlar ayrıca düzenlenemez veya silinemez.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Yorum Dizisini Salt Okunur Hale Getirme'; code-example-end]

Bu, kod olmadan, widget özelleştirme sayfasında, tüm bir alan adı veya sayfa için özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='Widget özelleştirme sayfasındaki yeni yanıtları önleme ayarı, bir alan adı veya sayfa için bir diziyi kilitler'; title='Yorum Dizisini Salt Okunur Hale Getirme' app-screenshot-end]

## Güncelleme!

Kasım 2022 itibarıyla, diziler yöneticiler ve moderatörler tarafından yanıt alanının üzerindeki üç nokta menüsü aracılığıyla **canlı** olarak kilitlenebilir veya kilidi açılabilir.

Bu, yeni yorumları engellerken hâlâ oy vermeye izin verir ve istenirse kullanıcıların yorumlarını silmesine izin verir; `readonly` bu şeylere izin vermez.

Bu, `Page` API'sindeki `isClosed` alanına karşılık gelir.