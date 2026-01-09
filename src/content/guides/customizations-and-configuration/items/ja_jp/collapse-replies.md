---
[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

デフォルトでは、トップレベルのコメントに対する返信が表示されます。

これを設定すると、ユーザーはトップレベルのコメントで「返信を表示」をクリックしないと子コメントが表示されないようにできます。

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

これはコードなしでウィジェットのカスタマイズページから変更できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

この設定は、最初に読み込まれるトップレベルのコメント数には影響しません。もしトップレベルのコメントが1件で、その下に29件の子コメントがある場合、この設定を有効にすると次のようになります：

- トップレベルのコメントが表示されます。
- このコメントの下に「返信を表示 (29)」が表示されます。

このオプションと組み合わせて、すべてのトップレベルのコメントを表示したい場合は、[starting page を -1 に設定する](#starting-page)。

---