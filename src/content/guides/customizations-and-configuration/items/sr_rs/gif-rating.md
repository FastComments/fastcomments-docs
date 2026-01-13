[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

По подразумевању, FastComments коментар виџет ће поставити `gif rating` на `pg`.

Доступне опције су `g`, `pg`, `pg-13` и `r`.

Ово се може подесити у коду или преко корисничког интерфејса. У коду то можемо урадити на следећи начин:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

У корисничком интерфејсу, ово ћете наћи под `Gif Picker Rating` све док `Disable Image Uploads?` није означено.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]