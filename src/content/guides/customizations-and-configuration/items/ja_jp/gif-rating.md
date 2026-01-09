---
[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

デフォルトでは、FastComments コメントウィジェットは `gif rating` を `pg` に設定します。

利用可能なオプションは `g`、`pg`、`pg-13`、および `r` です。

これはコードまたは UI で設定できます。コード内では次のようにします：

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

UI では、`Disable Image Uploads?` がチェックされていない限り、`Gif Picker Rating` の下にあります。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---