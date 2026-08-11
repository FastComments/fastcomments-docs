[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

より大規模なカスタムスタイリングプロジェクトでは、デフォルトのスタイリングを全く使用せず、クリーンな状態から始めることが望ましい場合があります。

すべてのデフォルトスタイリングは、**noStyles** パラメータを true に設定することで削除できます。以下のように：

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

これは、コードを使用せずに、ウィジェットカスタマイズページの「詳細オプション」セクションでカスタマイズできます：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='ウィジェットカスタマイズページの「詳細オプション」内で有効になっている「すべてのデフォルトスタイリングを無効にする」チェックボックス'; title='すべてのデフォルトスタイルを無効化' app-screenshot-end]