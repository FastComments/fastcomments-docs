[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Можемо омогућити подршку за спојлере подешавањем флага **enableSpoilers** на true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Ово се може урадити и без кода. На страници за прилагођавање виџета, погледајте опцију "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Када је текст обележен, и када се кликне на сада видљиво `SPOILER` дугме, текст ће бити замагљен док корисник не превуче курсор преко њега. За тамни режим радимо исту ствар, са другачијим
бојама које боље одговарају тамном режиму.

Ово је такође компатибилно са WYSIWYG уредником.