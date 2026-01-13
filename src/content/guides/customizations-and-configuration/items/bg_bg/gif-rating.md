[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

По подразбиране коментарният виджет на FastComments ще зададе `gif rating` на `pg`.

Достъпните опции са `g`, `pg`, `pg-13` и `r`.

Това може да се зададе в кода или чрез потребителския интерфейс. В кода можем да го направим по следния начин:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

В потребителския интерфейс това се намира под `Gif Picker Rating`, стига опцията `Disable Image Uploads?` да не е отметната.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]