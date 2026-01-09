[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Подразумевано, FastComments омогућава корисницима да блокирају друге кориснике. Блокирање корисника узрокује да његови коментари
буду маскирани, спречава обавештења између корисника и слично.

Може бити пожељно онемогућити ову функционалност. То се може урадити на следећи начин:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Ово се такође може урадити без кода, што такође омогућава исправну валидацију на серверској страни, путем корисничког интерфејса за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---