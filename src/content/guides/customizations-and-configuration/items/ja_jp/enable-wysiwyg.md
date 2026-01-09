[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments の書式機能は、テキストの周りに `<b></b>` のような目に見えるアンカータグを追加することで行われます。ツールバーをクリックするかショートカットを使うと、自動的にこれが適用されます。しかし、一部のコミュニティではアンカータグを使わずに書式を適用したい場合があります。これを
WYSIWYG（表示されるものがそのまま反映される）エディタを有効にする、という言い方をします。このエディタはデフォルトのものと見た目はまったく同じですが、目に見えるアンカータグを使わずにユーザーがテキストを太字や下線などにできるようにする追加のコードを読み込みます。

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

これはコードなしでも行えます。ウィジェットのカスタマイズページで、"Enable Advanced Formatting" オプションをご覧ください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]