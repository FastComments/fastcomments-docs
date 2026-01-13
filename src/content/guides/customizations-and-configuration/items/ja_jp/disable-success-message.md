[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメント投稿後に成功メッセージを表示します。これを無効にするには、次のようにします:

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = 'Disable Success Message'; code-example-end]

これはコードなしでも行えます。ウィジェットのカスタマイズページで:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; title='Disable Success Message' app-screenshot-end]