[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

コメント入力フィールドに入力できる最大文字数は、**maxCommentCharacterLength** パラメータで制限できます。

デフォルトは2000です。

画像のURLなどは文字数の判定に含まれません。

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

これはウィジェットのカスタマイズページで、コード不要でカスタマイズできます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]