---
[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

При отправке уведомлений по электронной почте или при отображении комментариев в пользовательских интерфейсах, таких как страница модерации, полезно иметь возможность ссылаться из комментария на страницу, где он размещён.

Если URL ID не всегда является идентификатором, то нам нужно хранить URL в другом месте. Для этого и существует свойство "url", определённое следующим образом.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Типичный сценарий использования — привязать поток комментариев к некоторому идентификатору, например статье, а затем ссылаться обратно на конкретную страницу, например:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL не очищается от распространённых маркетинговых параметров. По умолчанию, какой бы ни был URL текущей страницы, именно он сохраняется вместе с комментарием.

---