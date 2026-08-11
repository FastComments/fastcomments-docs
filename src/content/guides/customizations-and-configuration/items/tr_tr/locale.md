[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

Varsayılan olarak, FastComments yorum widget'ını kullanıcının sistem ve tarayıcı tarafından belirlenen yerel ayarda render eder.

Bir kullanıcı yorum yaptığında veya oturum açtığında, son kullandığı yerel ayarı günceller ve bunu e-posta gönderiminde de kullanırız.

Bu, yorum widget'ının kullanıcı için nasıl çevrileceğini etkiler. Yerel ayar, kullanıcının dili ve bölgesinden oluşur, bu yüzden yerel ayarı yapılandırmak genellikle kullanıcıya gösterilen metnin dilini değiştirir.

#### Arayüz Üzerinden

Bu, widget özelleştirme UI'sı kullanılarak tanımlanabilir. "Locale / Language" seçeneğine bakın:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='Widget özelleştirme sayfasında ziyaretçinin algılanan yerel ayarını geçersiz kılmak için Dil / Dil açılır menüsü'; title='Dil / Yerel Ayarı Değiştirme' app-screenshot-end]

#### Kod ile

Bu, istenen bir yerel ayar ile geçersiz kılınabilir.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Kullanıcının Dilini Manuel Olarak Tanımlama'; code-example-end]

### Desteklenen Diller ve Yerel Ayar Kodları

[Desteklenen dillerin tam listesini ve ilgili yerel ayar kodlarını burada bulabilirsiniz.](/guide-supported-languages.html#supported-languages)

### SSO Notu

SSO kullanıyorsanız, kullanıcının yerel ayarını kullanıcı nesnesine geçirmek isteyebilirsiniz, böylece e-postalar ve diğer öğeler onlar için doğru şekilde yerelleştirilir.

---