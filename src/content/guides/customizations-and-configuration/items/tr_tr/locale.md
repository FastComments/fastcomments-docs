[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Varsayılan olarak, FastComments yorum bileşenini kullanıcının sistem ve tarayıcısı tarafından belirlenen yerel ayara (locale) göre gösterir.

Bir kullanıcı yorum yaptığında veya giriş yaptığında, son kullandıkları locale bilgisini güncelliyoruz ve e-postaları gönderirken bunu kullanıyoruz.

Bu, yorum bileşeninin kullanıcı için nasıl çevrileceğini etkiler. Locale, kullanıcının dil ve bölgesinden oluşur; bu yüzden locale'i yapılandırmak genellikle kullanıcıya gösterilen metnin dilini değiştirir.

#### Arayüz Üzerinden

Bu, bileşen özelleştirme kullanıcı arayüzü kullanılarak tanımlanabilir. "Yerel Ayar / Dil" seçeneğine bakın:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Kod ile

Bu, istenen bir yerel ayar (locale) ile geçersiz kılınabilir.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Desteklenen Diller ve Yerel Ayar Kodları

[Desteklenen dillerin ve karşılık gelen locale kodlarının tam listesini burada bulabilirsiniz.](/guide-supported-languages.html#supported-languages)

### SSO Notu

SSO kullanıyorsanız, e-postaların ve diğer öğelerin kullanıcı için doğru şekilde yerelleştirilmesini sağlamak üzere kullanıcının locale bilgisini kullanıcı nesnesinde iletmek isteyebilirsiniz.