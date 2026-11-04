[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Bildirim e-postaları gönderirken veya moderasyon sayfası gibi kullanıcı arayüzlerinde yorumları görüntülerken, yorumdan bulunduğu sayfaya bağlantı verebilmek faydalıdır.

URL ID her zaman bir ID değilse, URL'yi başka bir yerde saklamamız gerekir. "url" özelliği tam olarak bunun içindir ve şu şekilde tanımlanır.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Özel Bir URL Tanımlama'; code-example-end]

Yaygın bir kullanım senaryosu, yorum dizisini bir makale gibi bir tanımlayıcıya bağlamak ve ardından belirli bir sayfaya geri bağlantı vermektir, örneğin:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Özel URL ve URL ID\'lerini Birlikte Tanımlama'; code-example-end]

URL, yaygın pazarlama parametrelerinden temizlenmez. Varsayılan olarak, mevcut sayfa URL'si ne ise, yorumla birlikte saklanan URL odur.