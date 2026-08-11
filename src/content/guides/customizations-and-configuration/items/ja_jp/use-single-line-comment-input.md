[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はユーザーが好きなだけの行数でコメントを入力でき、デフォルトの文字数制限まで許可します。

ただし、ユーザーがテキストを1行だけ入力できるように制限したい場合があります。例として、オンライン入札やライブチャットなど、FastComments を利用できるケースがあります。

次のように **useSingleLineCommentInput** フラグを有効にします：

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

これはコードなしでも設定できます。ウィジェットカスタマイズページで「Enable Single-Line Comment Input」セクションを確認してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='ウィジェットカスタマイズページでシングルラインコメント入力チェックボックスがオンになり、入力が1行に制限されます'; title='シングルラインコメント入力を有効にする' app-screenshot-end]

なお、各ページのコメントは各ソート方向ごとに事前に計算されているため、すべてのソート方向で同じパフォーマンスが得られます。