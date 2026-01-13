[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

コメントウィジェットの上部に表示されるコメント数はカスタマイズできます。

これには任意の文字列を設定でき、値 **[count]** はユーザーのロケールに合わせてローカライズされた件数に置き換えられます。

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

これはコードを書く必要はなく、ウィジェットのカスタマイズページで設定できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]