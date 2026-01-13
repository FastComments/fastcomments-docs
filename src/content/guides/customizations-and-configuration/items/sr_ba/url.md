[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Када шаљете обавјештења путем е-поште, или при приказивању коментара у корисничким интерфејсима као што је страница за модерацију, корисно је моћи повезати
из коментара на страницу на којој се налази.

Ако URL ID није увијек прави ID, онда морамо похранити URL негдје друго. За то служи својство "url", дефинисано на сљедећи начин.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Чест случај употребе је повезивање теме коментара са идентификатором, као што је чланак, а затим повезивање назад на одређену страницу, на примјер:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL се не чисти од уобичајених маркетиншких параметара. По подразумевању, који год је тренутни URL странице, тај URL се складишти уз коментар.