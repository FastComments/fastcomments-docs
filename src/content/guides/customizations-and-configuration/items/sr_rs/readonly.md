[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Коментарисање се може закључати тако да се не могу остављати нови коментари или гласови подешавањем флага readonly на true.

Коментари такође неће моћи да буду уређивани или избрисани.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Ово се може прилагодити без кода, на страници за прилагођавање виџета, за цео домен или страницу:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Ажурирање!

Од новембра 2022. године, теме могу бити закључане или откључане **у реалном времену** од стране администратора и модератора преко менија са три тачке изнад поља за одговор.

Ово ће спречити нове коментаре, али ће и даље дозвољавати гласање и омогућити корисницима да по жељи обришу своје коментаре, док `readonly` не дозвољава ове функције. 

Ово одговара пољу `isClosed` у `Page` API-ју.