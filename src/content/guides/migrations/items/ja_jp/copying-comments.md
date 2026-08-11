データを移動する必要がある場合、FastComments はページや記事間でコメントを移動するためのセルフサービスツールを提供しています。

以下はコメントコピー ページ フォームの外観です：

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='From URL ID フィールドと To URL ID および URL フィールドを含むコメントコピー フォーム'; title='コメントコピー フォーム' app-screenshot-end]

### "From" フィールドの入力

コメントをどこから移動するか決めるには、単に元の `URL ID` を知る必要があります。

コメントウィジェット設定で `urlId` の値を渡していない場合、これはページ URL の「クリーン」バージョンになります。

`URL ID` の値はエクスポートすることで確認できます。

### "To" フィールドの入力

コメントをどこへ移動するか決めるには、対象の `URL ID` と `URL` を知る必要があります。

`URL ID` はコメントが入るバケットになります。`URL` フィールドは、メールやモデレーションツールからコメントへ直接ナビゲートできるように使用されます。

#### WordPress

WordPress を使用している場合、マイグレーションツールの To/From `URL ID` フィールドに記事 ID を入力し、URL ではなく ID を使用します。