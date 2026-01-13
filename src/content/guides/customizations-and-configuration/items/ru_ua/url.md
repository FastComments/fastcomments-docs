[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

При отправке уведомлений по электронной почте, или при отображении комментариев в интерфейсах пользователя, таких как страница модерации, полезно иметь возможность ссылаться
из комментария на страницу, на которой он находится.

Если URL ID не всегда является идентификатором, то нам надо сохранить URL в другом месте. Именно для этого существует свойство "url", определённое следующим образом.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Обычный сценарий использования — привязать поток комментариев к какому‑то идентификатору, например статье, а затем ссылаться назад на конкретную страницу, например:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL не очищается от распространённых маркетинговых параметров. По умолчанию сохраняется тот URL текущей страницы, который есть в данный момент.

---