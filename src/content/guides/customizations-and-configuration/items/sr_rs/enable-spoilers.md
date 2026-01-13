[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Можемо омогућити подршку за спојлере подешавањем заставице **enableSpoilers** на true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Ово се такође може урадити и без кода. На страници за прилагођавање виџета, погледајте опцију "Омогући спојлере".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Када је текст обележен, и када се кликне на сада видљиво дугме `SPOILER`, текст ће бити маскиран док корисник не помери курсор преко њега. За тамни режим радимо исто, али са другачијим бојама које боље одговарају тамном режиму.

Ово је такође компатибилно са WYSIWYG уређивачем.