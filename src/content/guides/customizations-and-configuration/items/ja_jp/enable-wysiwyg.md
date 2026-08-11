[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments の書式機能は、テキストの周りに `<b></b>` のような可視のアンカータグを追加することで実現されます。ツールバーをクリックするかショートカットを使用すると、これが自動的に行われます。ただし、一部のコミュニティではアンカータグなしで書式設定を使用したい場合があります。これは WYSIWYG（What You See Is What You Get）エディタを有効にすることと呼ばれます。このエディタはデフォルトのものと見た目は全く同じですが、可視のアンカータグなしで太字、下線などを可能にする追加コードをロードします。

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'WYSIWYG 編集の有効化'; code-example-end]

コードを書かずにこれを行うこともできます。ウィジェットカスタマイズページで「高度な書式設定を有効にする」オプションを確認してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='WYSIWYG エディタを有効にするために「高度な書式設定を有効にする」チェックボックスがオンになっているウィジェットカスタマイズページ'; title='WYSIWYG を有効化' app-screenshot-end]