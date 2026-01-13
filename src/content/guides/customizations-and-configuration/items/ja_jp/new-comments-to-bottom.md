[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

デフォルトでは、新しいライブコメントはリアルタイムで投稿されるとコメント一覧の上部に表示されます。

このオプションを有効にすると、新しいライブコメントは代わりに一覧の下部に追加されます。これは、ユーザーがコメントスレッドを閲覧している間にコメントがライブで投稿されたときの表示方法に影響します。

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

With this setting enabled:
- 他のユーザーが投稿した新しいライブコメントはコメント一覧の下部に表示されます
- ユーザーは既存のコメントの下に新しいコメントがリアルタイムで表示されるのを確認できます
- これはライブコメントの更新にのみ影響し - 初回のページ読み込みには影響しません
- ユーザーが議論を追っているときに読み進める流れを維持するのに役立ちます

Note that this setting only affects where new live comments are placed as they arrive in real-time. It does not affect the initial sort order when the page loads.