[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments разработан для настройки, и шрифт, используемый нашими виджетами, не является исключением.

По умолчанию FastComments использует `system font stack` чтобы выглядеть максимально хорошо на широком спектре устройств.

Чтобы задать собственные шрифты, см. [документацию по Custom CSS](/guide-customizations-and-configuration.html#custom-css).

Там вы найдете способ задать пользовательский CSS, который позволит использовать нужные вам шрифты.

#### Как задать шрифт

Чтобы переопределить шрифт, мы рекомендуем задать ваш CSS с использованием селекторов `.fast-comments, textarea`. Например:

[inline-code-attrs-start title = 'Пример внешнего шрифта'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]