[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

По подразбиране, уиджетът за коментари FastComments ще зададе `gif rating` на `pg`.

Наличните опции са `g`, `pg`, `pg-13` и `r`.

Това може да се зададе в кода или чрез UI. В кода можем да го направим по следния начин:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

В UI ще го намерите под `Gif Picker Rating`, стига `Disable Image Uploads?` да не е отметнато.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Падащо меню Gif Picker Rating на страницата за персонализиране на уиджета, предлагащо g, pg, pg-13 и r'; title='Настройване на Gif Rating' app-screenshot-end]