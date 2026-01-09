[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Када се шаљу е-поруке са обавештењима, или када се коментари приказују у корисничким интерфејсима као што је страница за модерацију, корисно је моћи повезати коментар са страницом на којој се налази.

Ако URL ID није увек прави идентификатор, онда морамо да сачувамо URL негде друго. Зато постоји својство "url", које је дефинисано на следећи начин.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Чест случај употребе је повезивање нити коментара са идентификатором, као што је чланак, а затим повратно повезивање на одређену страницу, на пример:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL се не уклања од уобичајених маркетиншких параметара. По подразумеваној вредности, шта год да је тренутни URL странице, то је URL који се чува уз коментар.