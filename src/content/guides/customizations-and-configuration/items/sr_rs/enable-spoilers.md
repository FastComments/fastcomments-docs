[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Можемо омогућити подршку за спојлере постављањем **enableSpoilers** заставице на true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Омогућавање спојлера'; code-example-end]

Ово се такође може урадити без кода. На страници за прилагођавање виџета, погледајте опцију "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Страница за прилагођавање виџета са означеним пољетом Enable Spoilers да би се додао SPOILER дугме у уређивач'; title='Омогући спојлере' app-screenshot-end]

Када се текст означи, а сада видљиво дугме `SPOILER` кликне, текст ће бити замаскиран док корисник не пређе мишем преко њега. За тамни режим радимо исто, са различитим бојама које боље одговарају тамном режиму.

Ово је такође компатибилно са WYSIWYG уређивачем.