---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

コメント入力フィールドに入力できる最大文字数は、**maxCommentCharacterLength** パラメータで制限できます。

デフォルトは 2000 です。

画像 URL などは文字数の計算に含まれません。

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'コメント長さの制限'; code-example-end]

コードなしで、ウィジェットのカスタマイズページで設定できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='ウィジェットカスタマイズページの最大コメントサイズフィールド。コメントが含められる文字数の上限を設定するために使用されます。'; title='コメント長さの制限' app-screenshot-end]

---