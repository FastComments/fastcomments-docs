[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Коли надсилаються сповіщення електронною поштою або відображаються коментарі в інтерфейсах користувача, наприклад на сторінці модерації, корисно мати можливість посилатися з коментаря на сторінку, де він розміщений.

Якщо URL ID не завжди є ідентифікатором, то потрібно зберігати URL в іншому місці. Для цього служить властивість "url", яка визначена так:

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Поширений випадок використання — прив'язати тред коментарів до ідентифікатора, наприклад статті, а потім посилатися назад на конкретну сторінку, наприклад:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL не очищується від типових маркетингових параметрів. За замовчуванням зберігається саме той URL, який є на поточній сторінці.