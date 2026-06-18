デフォルトでは FastComments はコメント内の `\<iframe\>` を許可しません。メディア埋め込みを有効にすると、コメント投稿者は YouTube、Vimeo、SoundCloud、Spotify のような信頼されたプロバイダからの埋め込みコード（`<iframe>` スニペット）を貼り付けることができ、コメント内にインラインで表示されます。

セキュリティ上、これはクライアント側のウィジェット設定フラグではありません。これはサーバー側の設定であり、各コメントが保存される際に検証されるため、ページから有効化することはできません。組み込みの信頼できるプロバイダの一覧を指す iframe のみが許可され、その他の iframe は削除されます。

これはコード不要で、ウィジェットのカスタマイズページで行います:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### 独自プロバイダの追加

組み込みの信頼済みリストにないプロバイダの埋め込みを許可したい場合は、同じページの「Additional Embed Domains」フィールドにそのホスト名を追加してください。これらのホスト名は組み込みプロバイダに加えて許可されます。マッチングは完全一致なので、フルホスト名を含めてください（例: player.example.com）。リストに含めていないものはすべてブロックされたままです。

通常のコメント入力欄と WYSIWYG エディタの両方で埋め込みコードの貼り付けに対応しています。WYSIWYG エディタでは、埋め込みは削除可能なブロックとして挿入されます。