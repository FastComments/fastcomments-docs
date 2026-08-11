[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

コメントウィジェットの上部に表示されるコメント数はカスタマイズできます。

任意の文字列に置き換えることができ、値 **[count]** はユーザー向けにローカライズされたカウント値に置き換えられます。

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'コメント数テキストのカスタマイズ'; code-example-end]

コードなしで、ウィジェットのカスタマイズページでカスタマイズできます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='ウィジェットカスタマイズページのコメント数テキストフィールド。ここで [count] はリアルタイムの合計に置き換えられます'; title='コメント数テキストのカスタマイズ' app-screenshot-end]