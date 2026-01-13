[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

By default, the FastComments comment widget sets the `gif rating` to `pg`.

Available options are `g`, `pg`, `pg-13`, and `r`.

You can set this in the code or via the UI. In code, you can do it as follows:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

In the UI, you'll find this under `Gif Picker Rating` as long as `Disable Image Uploads?` is not checked.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]