[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

По подразумеваној вредности, FastComments видгет ће аутоматски прилагођавати вертикалну висину да уклопи све видљиве коментаре. Пагинација се остварује преко дугмета "Прикажи следеће" на крају тренутне странице, јер смо утврдили да је то интеракција која већини корисника делује најприродније.

Међутим, постоје случајеви када је пожељно бесконачно скроловање. На пример, ову функцију користимо у нашем Stream Chat производу.

Можемо сакрити дугмад "Прикажи следеће" и прећи на бесконачно скроловање тако што ћемо опцију **enableInfiniteScrolling** поставити на true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Ово такође захтева додатак прилагођеног CSS-а. Додајте прилагођени CSS за селектор `.comments` да бисте омогућили скроловање, на пример:

[inline-code-attrs-start title = 'Омогућавање бесконачног скроловања'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Пуни радни пример би изгледао овако:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

У горе наведеном примеру користимо својство `customCSS`, међутим препоручује се коришћење UI за конфигурацију видгета уместо тога из разлога перформанси. [Погледајте документацију о прилагођеном CSS-у.](/guide-customizations-and-configuration.html#custom-css)