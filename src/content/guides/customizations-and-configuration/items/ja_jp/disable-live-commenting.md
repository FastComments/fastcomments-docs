[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastCommentsはライブコメント機能が有効になっています。

これは、コメントスレッドのすべての閲覧者が同じ内容を見ることを意味します。

たとえば、コメントが追加された場合、そのコメントが表示されます。コメントが編集または削除された場合、
その編集や削除はスレッドの全ての閲覧者に対して反映されます。投票やすべてのモデレーション操作も同様です。

ただし、これを無効にすることもできます:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

これはコードを使わずに行うこともできます。ウィジェットのカスタマイズページで "Disable Live Commenting" セクションを参照してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]