[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

**enableSpoilers** フラグを true に設定することで、スポイラーサポートを有効にできます:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'スポイラーの有効化'; code-example-end]

コードを使用せずにこれを行うこともできます。ウィジェットカスタマイズページで「Enable Spoilers」オプションを確認してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='ウィジェットカスタマイズページで「Enable Spoilers」チェックボックスがオンになっており、エディタに SPOILER ボタンが追加されています'; title='スポイラーの有効化' app-screenshot-end]

テキストがハイライトされ、表示された `SPOILER` ボタンがクリックされると、テキストはマスクされ、ユーザーがマウスオーバーするまで隠れたままになります。ダークモードの場合は、同様の動作を行いますが、ダークモードにより適した異なる色を使用します。

これは WYSIWYG エディタでも互換性があります。