[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

За замовчуванням віджет коментарів FastComments встановлює `gif rating` у `pg`.

Доступні варіанти: `g`, `pg`, `pg-13` та `r`.

Це можна встановити в коді або через інтерфейс користувача. У коді це можна зробити наступним чином:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Встановити рейтинг GIF'; code-example-end]

У UI ви знайдете це під `Gif Picker Rating`, доки `Disable Image Uploads?` не позначено.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='Випадаюче меню Gif Picker Rating на сторінці налаштування віджета, що пропонує g, pg, pg-13 та r'; title='Налаштування рейтингу GIF' app-screenshot-end]