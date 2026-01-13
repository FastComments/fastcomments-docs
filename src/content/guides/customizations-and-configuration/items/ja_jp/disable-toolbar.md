[related-parameter-start name = 'disableToolbar'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はコメントを作成する際に、テキストの装飾やアップロード
画像のショートカットを提供するツールバーを表示します。

このツールバーは、コードまたはカスタマイズ UI で無効にできます。

[code-example-start config = {disableToolbar: true}; linesToHighlight = [6]; title = 'Disabling The Toolbar'; code-example-end]

これはコードを使わずに行うこともできます。ウィジェットのカスタマイズページで「返信ツールバーを無効にする」オプションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-toolbar']; selector = '.disable-toolbar'; title='Disabling The Toolbar' app-screenshot-end]