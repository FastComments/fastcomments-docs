---
[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメント入力ボックスとコメントスレッドを同時にレンダリングします。垂直方向のスペースを節約するため、ウィジェットが操作されるまで他の必須フィールドは非表示にします。

ただし、コメントウィジェットをボタンの背後に隠すこともできます。例えば:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; title='Click to Show Comments' app-screenshot-end]

ボタンはコメントが現在表示されているかどうかに応じて、異なる翻訳済みテキストを使用します。コメントが非表示の場合は `translations.SHOW_COMMENTS_BUTTON_TEXT` を使用します。コメントが表示されている場合は `translations.HIDE_COMMENTS_BUTTON_TEXT` を使用します。翻訳テキストには `[count]` というテキストを含めることができ、ローカライズされたカウントに置き換えられます。

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Click to Show or Hide Comments'; code-example-end]

これは `hideCommentsUnderCountTextFormat` 設定を置き換えるために設計されています。

カウントはコメントスレッドとリアルタイムで更新されます。コメントがない場合、ボタンは表示されません。

これは、カスタマイズルールを作成して「クリックしてコメントを表示」を有効にすることで、コード不要で有効化できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; title='Enable Click to Show Comments' app-screenshot-end]


---