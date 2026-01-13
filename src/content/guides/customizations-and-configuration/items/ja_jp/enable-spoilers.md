[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

スポイラー機能は**enableSpoilers**フラグをtrueに設定することで有効にできます:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

これはコードを使わずに行うこともできます。ウィジェットのカスタマイズページで、"Enable Spoilers" オプションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

テキストがハイライトされ、表示された `SPOILER` ボタンがクリックされると、ユーザーがマウスオーバーするまでテキストはマスクされます。ダークモードでも同じ処理を行いますが、異なる
色を使用してダークモードにより適合させます。

これはWYSIWYGエディタとも互換性があります。