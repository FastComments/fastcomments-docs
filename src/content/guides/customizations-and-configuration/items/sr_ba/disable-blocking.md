---
[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

По подразумеваној вредности, FastComments омогућава корисницима да блокирају друге кориснике. Блокирање корисника ће узроковати њихове коментаре
да буду маскирани, спречити обавештења између корисника, и тако даље.

Може бити пожељно онемогућити ову функционалност. То се може урадити на следећи начин:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Ово се такође може урадити без кода, што такође омогућава правилну валидацију на серверу, преко корисничког интерфејса за прилагођавање видгета:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---