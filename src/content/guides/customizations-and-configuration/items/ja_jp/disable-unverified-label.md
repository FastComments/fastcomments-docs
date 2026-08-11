[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments は、未確認のブラウザー セッションを持つユーザーに対して残されたコメントに「Unverified Comment」ラベルを表示します。未確認コメントについての詳細は[こちら](https://docs.fastcomments.com/guide-comment-vote-verification.html)をご覧ください。

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

さらに、この機能はコードを書かずにカスタマイズ UI で使用できます。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='「Disable Unverified Comment Label」チェックボックスがオンになっているウィジェットカスタマイズページ'; title='未確認ラベルを無効にする' app-screenshot-end]

---