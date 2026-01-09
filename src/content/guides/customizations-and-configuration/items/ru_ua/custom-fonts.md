[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments создан для кастомизации, и шрифт, который используют наши виджеты, не является исключением.

По умолчанию FastComments использует `system font stack`, чтобы выглядеть максимально хорошо на широком диапазоне устройств.

Чтобы задать собственные шрифты, см. [Документация по пользовательскому CSS](/guide-customizations-and-configuration.html#custom-css).

Там вы найдёте способ определить пользовательский CSS, который позволит вам задать желаемые шрифты.

#### Как задать шрифт

Чтобы переопределить шрифт, мы рекомендуем определять ваш CSS с использованием селекторов `.fast-comments, textarea`. Например:

[inline-code-attrs-start title = 'Пример внешнего шрифта'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]