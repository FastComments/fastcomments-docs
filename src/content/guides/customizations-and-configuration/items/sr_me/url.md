[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Када шаљете обавештења путем е-поште, или при приказивању коментара у корисничким интерфејсима као што је страница за модерирање, корисно је имати могућност повезивања
коментара са страницом на којој се налази.

Ако URL ID није увек стварни ID, онда морамо негде друго да сачувамо URL. За то служи својство "url", дефинисано на следећи начин.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Чест случај употребе је повезивање нити коментара са идентификатором, као што је чланак, и затим повратно повезивање на одређену страницу, на пример:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL се не чисти од уобичајених маркетиншких параметара. По подразумеваној вредности, било који URL тренутне странице ће бити сачуван уз коментар.