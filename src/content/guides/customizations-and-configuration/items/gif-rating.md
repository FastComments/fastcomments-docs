[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

By default, the FastComments comment widget will set a `gif rating` of `pg`.

Available options are `g`, `pg`, `pg-13`, and `r`.

This can be set in the code or via the UI. In the code we can do it as follows:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

In the UI, you'll find this under `Gif Picker Rating` as long as `Disable Image Uploads?` is not checked.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]
