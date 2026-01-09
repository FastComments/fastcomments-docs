[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

За замовчуванням віджет коментарів FastComments встановлює `gif rating` рівнем `pg`.

Доступні опції: `g`, `pg`, `pg-13` та `r`.

Це можна встановити в коді або через інтерфейс користувача. У коді це можна зробити так:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

В інтерфейсі це знаходиться в розділі `Gif Picker Rating`, якщо не встановлено прапорець `Disable Image Uploads?`.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---