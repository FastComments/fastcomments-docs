[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments は、ユーザーに残されたコメントで、その
ブラウザセッションが未検証である場合に「未検証のコメント」ラベルを表示します。未検証のコメントについての詳細は[こちら](https://docs.fastcomments.com/guide-comment-vote-verification.html)をご覧ください。

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

さらに、この機能はコードを記述せずにカスタマイズ UI で使用できます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---