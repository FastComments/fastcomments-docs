[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

За замовчуванням віджет FastComments автоматично змінює свою висоту, щоб вмістити всі видимі коментарі. Пагінація реалізується за допомогою кнопки "Показати наступні"
в кінці поточної сторінки, оскільки ми виявили, що така взаємодія найбільше подобається більшості користувачів.

Однак є випадки, коли віддають перевагу нескінченному прокручуванню. Наприклад, ми використовуємо цю функцію в нашому продукті Stream Chat.

Ми можемо приховати кнопки "Показати наступні" і переключитися на нескінченне прокручування, встановивши прапорець **enableInfiniteScrolling** у true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Це також вимагає додавання власного CSS. Додайте кастомні стилі для селектора `.comments`, щоб увімкнути прокручування, наприклад:

[inline-code-attrs-start title = 'Увімкнення нескінченного прокручування'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Повний робочий приклад буде таким:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

У наведеному вище прикладі ми використовуємо властивість `customCSS`, однак для причин продуктивності рекомендується замість цього використовувати інтерфейс налаштування віджета. [Див. документацію з Custom CSS.](/guide-customizations-and-configuration.html#custom-css)

---