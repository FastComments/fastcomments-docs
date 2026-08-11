[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

デフォルトでは、FastComments はライブコメント機能が有効になっています。

これは、コメントスレッドのすべての閲覧者が同じ内容を見ることになることを意味します。

例えば、コメントが追加された場合、そのコメントが表示されます。コメントが編集または削除された場合、  
そのコメントがスレッドのすべての閲覧者に対して編集または削除されます。投票やすべてのモデレーションアクションも同様です。

ただし、これを無効にすることができます。

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'ライブコメントの無効化'; code-example-end]

コードを使用せずにこれを行うこともできます。ウィジェットカスタマイズページで「ライブコメントの無効化」セクションを確認してください。

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='ウィジェットカスタマイズページのライブコメント無効化セクション、リアルタイムスレッド更新をオフにします'; title='ライブコメントの無効化' app-screenshot-end]