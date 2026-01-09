[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

readonly bayrağı true olarak ayarlandığında yorum yapma kilitlenebilir, böylece yeni yorumlar veya oylar bırakılamaz.

Yorumlar ayrıca düzenlenemeyecek veya silinemeyecek.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Bu, tüm bir alan adı veya sayfa için, kod olmadan widget özelleştirme sayfasında özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Güncelleme!

Kasım 2022 itibarıyla, yorum dizileri yöneticiler ve moderatörler tarafından yanıt alanının üzerindeki üç nokta menüsü aracılığıyla **canlı** olarak kilitlenebilir veya kilidi açılabilir.

Bu, yeni yorumları engeller; yine de oy vermeye ve kullanıcıların istedikleri takdirde yorumlarını silmelerine izin verir; oysa `readonly` bunlara izin vermez. 

Bu, `Page` API'sindeki `isClosed` alanına karşılık gelir.