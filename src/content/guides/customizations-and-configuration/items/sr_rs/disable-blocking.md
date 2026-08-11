---
[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments омогућава корисницима да блокирају друге кориснике. Блокирање корисника ће довести до маскирања њихових коментара, спречава обавештења између корисника, и тако даље.

Може бити жељено онемогућити ову функционалност. То се може урадити на следећи начин:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Onemogući blokiranje'; code-example-end]

Ово се такође може урадити без кода, што такође омогућава исправну валидацију на серверској страни, преко корисничког интерфејса за прилагођавање виџета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Опција за онемогућавање блокирања у корисничком интерфејсу прилагођавања виџета, која спречава кориснике да блокирају једни друге'; title='Onemogući blokiranje' app-screenshot-end]

---