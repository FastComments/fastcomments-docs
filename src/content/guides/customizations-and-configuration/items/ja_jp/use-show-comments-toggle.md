[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメント入力ボックスとコメントスレッドを同時に表示します。垂直方向のスペースを節約するため、ウィジェットが操作されるまで他の必須フィールドは非表示になります。

ただし、コメントウィジェットはボタンの背後に隠すことができます。例として:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='コメントウィジェットがボタンの背後に折りたたまれ、読者がクリックするまでコメント数を表示します'; title='コメントを表示するにはクリック' app-screenshot-end]

ボタンは、コメントが現在表示されているかどうかに応じて異なる翻訳テキストを使用します。コメントが非表示の場合は `translations.SHOW_COMMENTS_BUTTON_TEXT` を使用し、コメントが表示されている場合は `translations.HIDE_COMMENTS_BUTTON_TEXT` を使用します。翻訳テキストには `[count]` を含めることができ、ローカライズされた件数に置き換えられます。

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'コメントの表示または非表示を切り替えるにはクリック'; code-example-end]

これは `hideCommentsUnderCountTextFormat` 設定の代わりに使用することを意図しています。

件数はコメントスレッドとリアルタイムで更新されます。コメントがない場合、ボタンは表示されません。

コードを書かずに、カスタマイズルールを作成し「Click to Show Comments」を有効にすることで有効化できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='ウィジェットカスタマイズページで、コメント表示のチェックボックスがカスタマイズルールでオンになっている'; title='クリックでコメント表示を有効にする' app-screenshot-end]