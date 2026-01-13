[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments призначений для налаштування, і шрифт, який використовують наші віджети, не є винятком.

За замовчуванням FastComments використовує `system font stack`, щоб виглядати якомога краще на широкому діапазоні пристроїв.

Щоб задати власні шрифти, дивіться [Документація з Custom CSS](/guide-customizations-and-configuration.html#custom-css).

Там ви знайдете спосіб визначити кастомний CSS, що дозволить вам задати бажані шрифти.

#### Як задати шрифт

Щоб перевизначити шрифт, ми рекомендуємо визначати ваш CSS, використовуючи селектори `.fast-comments, textarea`. Наприклад:

[inline-code-attrs-start title = 'Приклад зовнішнього шрифту'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---