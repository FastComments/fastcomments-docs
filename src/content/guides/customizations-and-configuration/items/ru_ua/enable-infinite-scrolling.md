[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

По умолчанию виджет FastComments автоматически изменяет свой вертикальный размер, чтобы вмещать все видимые комментарии. Постраничная навигация выполняется с помощью кнопки "View Next" в конце текущей страницы — это взаимодействие, которое, как мы обнаружили, большинству пользователей кажется наиболее удобным.

Тем не менее, в некоторых случаях предпочитают бесконечную прокрутку. Например, мы используем эту функцию в нашем продукте Stream Chat.

Мы можем скрыть кнопки "View Next" и переключиться на бесконечную прокрутку, установив флаг **enableInfiniteScrolling** в true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Также требуется добавить пользовательский CSS. Добавьте пользовательский CSS для селектора `.comments`, чтобы включить прокрутку, например:

[inline-code-attrs-start title = 'Включение бесконечной прокрутки'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Полный рабочий пример будет выглядеть так:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

В приведённом выше примере мы используем свойство `customCSS`, однако для повышения производительности рекомендуется использовать UI конфигурации виджета. [Смотрите документацию по Custom CSS.](/guide-customizations-and-configuration.html#custom-css)

---