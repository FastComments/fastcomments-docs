[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

より大規模なカスタムスタイリングのプロジェクトでは、最初からクリーンな状態で始め、デフォルトのスタイリングをまったく使用しないほうが望ましい場合があります。

すべてのデフォルトスタイリングは、**noStyles** パラメータを true に設定することで削除できます。以下のように設定します:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

これはコードを使わずに、ウィジェットのカスタマイズページの「詳細オプション」でカスタマイズできます:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]