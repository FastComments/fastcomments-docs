[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

デフォルトでは、トップレベルのコメントへの返信が表示されます。

これを設定すると、ユーザーはトップレベルのコメントで「Show Replies」をクリックして子コメントを表示する必要があります。

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

コードを使用せずに、ウィジェットカスタマイズページでカスタマイズできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='ウィジェットカスタマイズ UI の「Collapse replies」オプションで、子コメントを「Show Replies」リンクの背後に隠します'; title='返信を折りたたむ' app-screenshot-end]

この設定は、最初に読み込まれるトップレベルコメントの数には影響しません。この設定が有効な場合、トップレベルコメントが1つで子コメントが29件あると、次のようになります。

- トップレベルのコメントを見る。
- このコメントの下に「Show Replies (29)」が表示されます。

このオプションと組み合わせてすべてのトップレベルコメントを表示したい場合は、[starting page to -1](#starting-page) を設定してください。