[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

По умолчанию виджет FastComments автоматически изменяет высоту, чтобы вместить все видимые комментарии. Разбиение на страницы реализовано с помощью кнопки «Показать далее» в конце текущей страницы — мы обнаружили, что такое взаимодействие наиболее удобно для большинства пользователей.

Однако в некоторых случаях предпочитают бесконечную прокрутку. Например, мы используем эту функцию в продукте Stream Chat.

Мы можем скрыть кнопки «Показать далее» и переключиться на бесконечную прокрутку, установив флаг **enableInfiniteScrolling** в true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Для этого также требуется добавить пользовательский CSS. Добавьте пользовательский CSS для селектора `.comments`, чтобы включить прокрутку, например:

[inline-code-attrs-start title = 'Включение бесконечной прокрутки'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Полный рабочий пример будет выглядеть так:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

В приведённом выше примере мы используем свойство `customCSS`, однако для повышения производительности рекомендуется использовать интерфейс настройки виджета. [См. документацию по пользовательскому CSS.](/guide-customizations-and-configuration.html#custom-css)