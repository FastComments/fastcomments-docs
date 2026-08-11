[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

デフォルトでは、FastComments のコメントウィジェットは `gif rating` を `pg` に設定します。

利用可能なオプションは `g`、`pg`、`pg-13`、および `r` です。

この設定はコードまたは UI で行うことができます。コードでは次のように設定できます：

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Gif 評価の設定'; code-example-end]

UI では、`Disable Image Uploads?` がチェックされていない限り、`Gif Picker Rating` の下にこの設定があります。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='ウィジェットカスタマイズページの Gif Picker Rating ドロップダウンで、g、pg、pg-13、r を提供'; title='Gif 評価の設定' app-screenshot-end]