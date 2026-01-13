---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Можемо омогућити подршку за спојлере постављањем ознаке **enableSpoilers** на true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Ово се може учинити и без кода. На страници за прилагођавање виџета погледајте опцију "Омогући спојлере".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Када је текст означен, и када се кликне на сада видљиво дугме `SPOILER`, текст ће бити замагљен док корисник не превуче курсор преко њега. За тамни режим радимо исто, са различитим
бојама које боље одговарају тамном режиму.

Ово је такође компатибилно са WYSIWYG уредником.

---