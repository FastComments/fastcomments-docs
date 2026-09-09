[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

デフォルトでは、ユーザーがコメントを入力し、送信前にページをリロードしたり、タブを閉じたり、別のページへ移動したりすると、下書きは黙って失われます。

**warnOnUnsavedComment** を true に設定すると、コメントボックスや編集中のテキストが残っている間にページを離れようとした際、ブラウザがユーザーに確認を求めます。コメントが送信されるとテキストはクリアされるため、プロンプトは表示されません。

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = '未保存コメントの警告'; code-example-end]

このプロンプトはブラウザ固有のダイアログを使用します。最新のブラウザは独自の文言を表示し、カスタムテキストは無視されるため、メッセージをカスタマイズすることはできません。

このオプションは必要に応じて小さな拡張機能をロードするため、無効にしているサイトのウィジェットには何も追加されません。