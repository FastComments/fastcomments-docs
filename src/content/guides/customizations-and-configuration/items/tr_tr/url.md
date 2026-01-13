[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Bildirim e-postaları gönderirken, veya moderasyon sayfası gibi kullanıcı arayüzlerinde yorumları render ederken, yorumdan bulunduğu sayfaya
bağlantı verebilmek faydalıdır.

If URL ID isn't always an ID, then we have to store the URL some place else. That's what the "url" property is for, defined as follows.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Yaygın bir kullanım, yorum dizisini bir makale gibi bir kimliğe bağlamak ve ardından örneğin belirli bir sayfaya geri bağlamaktır:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

The URL does not get cleaned of common marketing parameters. By default, whatever the current page URL is, is the URL stored with the comment.