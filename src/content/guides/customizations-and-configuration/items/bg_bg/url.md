[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Когато изпращате уведомителни имейли, или визуализирате коментари в потребителски интерфейси като страницата за модериране, е полезно да можете да направите връзка
от коментара към страницата, на която се намира.

Ако URL ID не винаги е реално ID, тогава трябва да съхраним URL някъде другаде. За това служи свойството "url", дефинирано по следния начин.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Често срещан случай на употреба е свързването на нишката с коментари със идентификатор, например статия, и след това връзка обратно към конкретна страница, например:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL адресът не се почиства от общи маркетингови параметри. По подразбиране какъвто е текущият URL на страницата, такъв се съхранява с коментара.

---