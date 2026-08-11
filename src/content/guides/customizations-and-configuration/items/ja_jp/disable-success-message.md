---
[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメント投稿後に成功メッセージを表示します。これを無効にするには、以下のようにします：

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = '成功メッセージを無効にする'; code-example-end]

コードを使用せずに行うこともできます。ウィジェットのカスタマイズページで：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; alt='コメント投稿後の確認メッセージを非表示にする「成功メッセージを無効にする」チェックボックスがオンになっているウィジェットカスタマイズページ'; title='成功メッセージを無効にする' app-screenshot-end]

---