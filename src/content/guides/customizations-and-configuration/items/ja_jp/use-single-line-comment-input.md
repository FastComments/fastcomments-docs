[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーがデフォルトの文字数制限内で、好きなだけ行数のあるコメントを入力できるようになっています。

しかし、ユーザーの入力を1行のテキストのみに制限したい場合もあります。例えば、オンライン入札やライブチャットなど、FastComments
で使用できるケースがあります。

We enable the **useSingleLineCommentInput** flag as follows:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

これはコードを使わずに行うこともできます。ウィジェットのカスタマイズページで、「シングルラインのコメント入力を有効にする」セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

注意：各ページのコメントは、各ソート方向ごとに事前に計算されるため、すべてのソート方向でパフォーマンスは同じです。

---