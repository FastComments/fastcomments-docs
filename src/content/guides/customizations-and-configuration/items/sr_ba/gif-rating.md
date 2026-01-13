[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

По подразумеваној поставци, FastComments коментарски виџет ће поставити `gif rating` на `pg`.

Доступне опције су `g`, `pg`, `pg-13` и `r`.

Ово се може подесити у коду или преко корисничког интерфејса. У коду можемо то урадити на следећи начин:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

У корисничком интерфејсу ово ћете наћи под `Gif Picker Rating`, под условом да опција `Disable Image Uploads?` није означена.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]